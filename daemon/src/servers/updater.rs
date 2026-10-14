use anyhow::{Result, bail};
use futures_util::StreamExt;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

static ACTIVE_UI_CLIENTS: AtomicUsize = AtomicUsize::new(0);
static IS_UPDATING: AtomicBool = AtomicBool::new(false);

const REPO_LATEST_API: &str = "https://api.github.com/repos/iMAboud/goxlr-utility/releases/latest";

pub fn inc_ui_client() {
    ACTIVE_UI_CLIENTS.fetch_add(1, Ordering::SeqCst);
}

pub fn dec_ui_client() {
    ACTIVE_UI_CLIENTS.fetch_sub(1, Ordering::SeqCst);
}

pub fn has_ui_client() -> bool {
    ACTIVE_UI_CLIENTS.load(Ordering::SeqCst) > 0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateState {
    pub status: String, // "idle", "checking", "available", "downloading", "installing", "error"
    pub progress: f32,  // 0.0 to 100.0
    pub current_version: String,
    pub latest_version: Option<String>,
    pub download_url: Option<String>,
    pub release_url: Option<String>,
    pub release_notes: Option<String>,
    pub error: Option<String>,
}

lazy_static::lazy_static! {
    pub static ref UPDATE_STATE: Arc<RwLock<UpdateState>> = Arc::new(RwLock::new(UpdateState {
        status: String::from("idle"),
        progress: 0.0,
        current_version: String::from(crate::VERSION),
        latest_version: None,
        download_url: None,
        release_url: None,
        release_notes: None,
        error: None,
    }));
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    #[allow(dead_code)]
    size: u64,
}

pub fn is_newer_version(current: &str, candidate: &str) -> bool {
    let parse_ver = |s: &str| -> Vec<u64> {
        let clean = s.trim_start_matches(|c| c == 'v' || c == 'V');
        clean
            .split(|c: char| c == '.' || c == '-')
            .filter_map(|part| part.parse::<u64>().ok())
            .collect()
    };

    let cur = parse_ver(current);
    let cand = parse_ver(candidate);

    for i in 0..cur.len().max(cand.len()) {
        let cur_val = cur.get(i).copied().unwrap_or(0);
        let cand_val = cand.get(i).copied().unwrap_or(0);
        if cand_val > cur_val {
            return true;
        } else if cand_val < cur_val {
            return false;
        }
    }
    false
}

pub async fn get_update_state() -> UpdateState {
    UPDATE_STATE.read().await.clone()
}

pub async fn check_for_updates() -> Result<UpdateState> {
    {
        let mut state = UPDATE_STATE.write().await;
        if state.status == "downloading" || state.status == "installing" {
            return Ok(state.clone());
        }
        state.status = String::from("checking");
        state.error = None;
    }

    let client = reqwest::Client::builder()
        .user_agent("goxlr-utility-updater")
        .timeout(Duration::from_secs(12))
        .build()?;

    let response = client
        .get(REPO_LATEST_API)
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await;

    let res = match response {
        Ok(r) => r,
        Err(e) => {
            let mut state = UPDATE_STATE.write().await;
            state.status = String::from("error");
            state.error = Some(format!("Check failed: {}", e));
            return Ok(state.clone());
        }
    };

    if !res.status().is_success() {
        let mut state = UPDATE_STATE.write().await;
        state.status = String::from("error");
        state.error = Some(format!("GitHub returned HTTP {}", res.status()));
        return Ok(state.clone());
    }

    let text = res.text().await?;
    let release: GithubRelease = match serde_json::from_str(&text) {
        Ok(rel) => rel,
        Err(e) => {
            let mut state = UPDATE_STATE.write().await;
            state.status = String::from("error");
            state.error = Some(format!("Failed to parse release: {}", e));
            return Ok(state.clone());
        }
    };

    // Find the executable labeled "goxlr.exe" (case-insensitive) or GoXLR.exe
    let exe_asset = release.assets.iter().find(|a| {
        let name_lower = a.name.to_lowercase();
        name_lower == "goxlr.exe" || (name_lower.contains("goxlr") && name_lower.ends_with(".exe"))
    });

    let is_newer = is_newer_version(crate::VERSION, &release.tag_name);

    let mut state = UPDATE_STATE.write().await;
    state.current_version = String::from(crate::VERSION);
    state.latest_version = Some(release.tag_name.clone());
    state.release_url = Some(release.html_url.clone());
    state.release_notes = release.body.clone();

    if is_newer && exe_asset.is_some() {
        let asset = exe_asset.unwrap();
        state.status = String::from("available");
        state.download_url = Some(asset.browser_download_url.clone());
        info!(
            "Update available: {} (URL: {})",
            release.tag_name, asset.browser_download_url
        );
    } else {
        state.status = String::from("idle");
        state.download_url = None;
        info!("Utility is up to date (current: v{})", crate::VERSION);
    }

    Ok(state.clone())
}

#[derive(Debug, Deserialize)]
pub struct InstallRequest {
    pub download_url: Option<String>,
    pub version: Option<String>,
}

pub async fn start_update(req: Option<InstallRequest>) -> Result<UpdateState> {
    if IS_UPDATING.swap(true, Ordering::SeqCst) {
        return Ok(get_update_state().await);
    }

    let (url, version) = {
        let state = UPDATE_STATE.read().await;
        let url = req
            .as_ref()
            .and_then(|r| r.download_url.clone())
            .or_else(|| state.download_url.clone());
        let ver = req
            .as_ref()
            .and_then(|r| r.version.clone())
            .or_else(|| state.latest_version.clone());
        (url, ver)
    };

    let Some(download_url) = url else {
        IS_UPDATING.store(false, Ordering::SeqCst);
        bail!("No download URL available");
    };

    tokio::spawn(async move {
        if let Err(e) = run_download_and_install(&download_url, version.as_deref()).await {
            error!("Update failed: {}", e);
            let mut state = UPDATE_STATE.write().await;
            state.status = String::from("error");
            state.error = Some(e.to_string());
            IS_UPDATING.store(false, Ordering::SeqCst);
        }
    });

    Ok(get_update_state().await)
}

async fn run_download_and_install(download_url: &str, target_version: Option<&str>) -> Result<()> {
    info!("Starting update download from: {}", download_url);
    {
        let mut state = UPDATE_STATE.write().await;
        state.status = String::from("downloading");
        state.progress = 0.0;
        state.error = None;
        if let Some(v) = target_version {
            state.latest_version = Some(v.to_string());
        }
    }

    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("GoXLR_update.exe");

    let client = reqwest::Client::builder()
        .user_agent("goxlr-utility-updater")
        .timeout(Duration::from_secs(300))
        .build()?;

    let response = client.get(download_url).send().await?;
    if !response.status().is_success() {
        bail!("Download failed with status: {}", response.status());
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    let mut dest_file = File::create(&temp_file_path)?;
    let mut stream = response.bytes_stream();

    let mut last_progress_report = 0.0f32;

    while let Some(chunk) = stream.next().await {
        let data = chunk?;
        dest_file.write_all(&data)?;
        downloaded += data.len() as u64;

        if total_size > 0 {
            let pct = ((downloaded as f64 / total_size as f64) * 100.0) as f32;
            let clamped = pct.clamp(0.0, 99.5);
            if (clamped - last_progress_report).abs() >= 1.0 {
                last_progress_report = clamped;
                let mut state = UPDATE_STATE.write().await;
                state.progress = clamped;
            }
        }
    }

    dest_file.flush()?;
    drop(dest_file);

    let meta = std::fs::metadata(&temp_file_path)?;
    if meta.len() < 1024 * 1024 {
        bail!(
            "Downloaded file appears incomplete (size: {} bytes)",
            meta.len()
        );
    }

    info!(
        "Download complete ({} bytes). Staging installer swap...",
        meta.len()
    );

    {
        let mut state = UPDATE_STATE.write().await;
        state.status = String::from("installing");
        state.progress = 100.0;
    }

    // Determine target installation directory
    let program_files =
        env::var("ProgramFiles").unwrap_or_else(|_| String::from(r"C:\Program Files"));
    let install_dir = PathBuf::from(&program_files).join("GoXLR Utility");

    let current_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| install_dir.clone());

    spawn_detached_updater(&temp_file_path, &install_dir, &current_dir)?;

    // Give helper script process 400ms to spin up, then exit gracefully
    tokio::time::sleep(Duration::from_millis(400)).await;
    std::process::exit(0);
}

fn spawn_detached_updater(
    temp_file: &Path,
    install_dir: &Path,
    current_dir: &Path,
) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use std::process::Command;

        let script = format!(
            "Start-Sleep -Milliseconds 1200; \
             taskkill.exe /F /IM goxlr-daemon.exe /IM goxlr-utility-ui.exe /IM goxlr-launcher.exe /IM GoXLR.exe 2>&1 | Out-Null; \
             Start-Sleep -Milliseconds 600; \
             $tmp = '{tmp}'; \
             $destDir = '{inst}'; \
             $curDir = '{cur}'; \
             if (Test-Path $tmp) {{ \
                 $ranInstaller = $false; \
                 try {{ \
                     $p = Start-Process -FilePath $tmp -ArgumentList '--silent' -PassThru -Wait -ErrorAction Stop; \
                     if ($p.ExitCode -eq 0) {{ $ranInstaller = $true }}; \
                 }} catch {{ \
                     $ranInstaller = $false; \
                 }} \
                 if (-not $ranInstaller) {{ \
                     if (-not (Test-Path $destDir)) {{ New-Item -ItemType Directory -Path $destDir -Force -ErrorAction SilentlyContinue | Out-Null }}; \
                     $targetExe = Join-Path $destDir 'GoXLR.exe'; \
                     Copy-Item -Path $tmp -Destination $targetExe -Force -ErrorAction SilentlyContinue; \
                     $goxlrLower = Join-Path $destDir 'goxlr.exe'; \
                     Copy-Item -Path $tmp -Destination $goxlrLower -Force -ErrorAction SilentlyContinue; \
                     if ($curDir -and $curDir -ne $destDir) {{ \
                         Copy-Item -Path $tmp -Destination (Join-Path $curDir 'GoXLR.exe') -Force -ErrorAction SilentlyContinue; \
                         Copy-Item -Path $tmp -Destination (Join-Path $curDir 'goxlr.exe') -Force -ErrorAction SilentlyContinue; \
                     }}; \
                     Start-Process -FilePath $targetExe -ArgumentList '--silent'; \
                 }} \
                 Start-Sleep -Seconds 6; \
                 Remove-Item -Path $tmp -Force -ErrorAction SilentlyContinue; \
             }}",
            tmp = temp_file.to_string_lossy().replace('\'', "''"),
            inst = install_dir.to_string_lossy().replace('\'', "''"),
            cur = current_dir.to_string_lossy().replace('\'', "''"),
        );

        Command::new("powershell.exe")
            .args(["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-Command", &script])
            .creation_flags(0x08000000 | 0x00000008) // CREATE_NO_WINDOW | DETACHED_PROCESS
            .spawn()?;

        info!("Detached updater spawned successfully.");
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    {
        bail!("Automatic in-place updater is currently supported on Windows.");
    }
}

pub fn init_startup_check() {
    tokio::spawn(async {
        // Wait 12 seconds after daemon starts to ensure UI connection has registered if launched interactively
        tokio::time::sleep(Duration::from_millis(12000)).await;
        info!("Running startup background update check...");
        if let Ok(state) = check_for_updates().await {
            if state.status == "available" {
                if !has_ui_client() {
                    info!("No UI client active; starting silent background auto-update...");
                    let _ = start_update(None).await;
                } else {
                    info!("UI client is active; update marked available for UI presentation.");
                }
            }
        }
    });
}
