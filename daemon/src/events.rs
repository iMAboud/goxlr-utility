// This file primarily handles 'global' events which may occur inside the daemon from a potential
// variety of sources, which affect other parts of the daemon.

use crate::primary_worker::DeviceStateChange;
use crate::{SettingsHandle, Shutdown};
use goxlr_ipc::{HttpSettings, PathTypes};
use log::{debug, warn};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::{select, signal};

#[derive(Debug)]
#[allow(dead_code)]
pub enum EventTriggers {
    TTSMessage(String),
    Stop(bool),
    Sleep(oneshot::Sender<()>),
    Wake(oneshot::Sender<()>),
    Lock,
    Unlock,
    Open(PathTypes),
    Activate,
    OpenUi,
    DevicesStopped,
}

#[derive(Clone)]
pub struct DaemonState {
    pub show_tray: Arc<AtomicBool>,
    pub http_settings: HttpSettings,

    // TTS Output
    pub tts_sender: Sender<String>,

    // Shutdown Handlers
    pub shutdown: Shutdown,
    pub shutdown_blocking: Arc<AtomicBool>,

    // Settings Handle..
    pub settings_handle: SettingsHandle,
}

pub async fn spawn_event_handler(
    state: DaemonState,
    mut rx: Receiver<EventTriggers>,
    device_state_tx: Sender<DeviceStateChange>,
) {
    let mut triggered_device_stop = false;
    debug!("Starting Event Loop..");
    loop {
        select! {
            Ok(()) = signal::ctrl_c() => {
                debug!("Shutdown Phase 1 Triggered..");

                // Ctrl+C is a generic capture, although we should also check for SIGTERM under Linux..
                if !triggered_device_stop {
                    triggered_device_stop = true;
                    let _ = device_state_tx.send(DeviceStateChange::Shutdown(false)).await;
                }
            },
            Some(event) = rx.recv() => {
                match event {
                    EventTriggers::TTSMessage(message) => {
                        let _ = state.tts_sender.send(message).await;
                    }
                    EventTriggers::Stop(avoid_write) => {
                        if !triggered_device_stop {
                            debug!("Shutdown Phase 1 Triggered..");
                            triggered_device_stop = true;
                            let _ = device_state_tx.send(DeviceStateChange::Shutdown(avoid_write)).await;
                        } else {
                            debug!("Shutdown Phase 1 already in Progress");
                        }
                    }
                    EventTriggers::DevicesStopped => {
                        debug!("Shutdown Phase 2 Triggered..");

                        // This hits after devices have been stopped..
                        state.shutdown.trigger();
                        state.shutdown_blocking.store(true, Ordering::Relaxed);
                        break;
                    }

                    // In the case of Sleep / Wake, code elsewhere is going to be managing the
                    // things like inhibitors, so we need to pass on a sender so they can be
                    // notified when actions have been completed.
                    EventTriggers::Sleep(sender) => {
                        let _ = device_state_tx.send(DeviceStateChange::Sleep(sender)).await;
                    }
                    EventTriggers::Wake(sender) => {
                        let _ = device_state_tx.send(DeviceStateChange::Wake(sender)).await;
                    }
                    EventTriggers::Lock => {
                        debug!("Received Screen Lock Event..");
                    }
                    EventTriggers::Unlock => {
                        debug!("Received Screen Unlock Event");
                    }

                    EventTriggers::Open(path_type) => {
                        if let Err(error) = open::that(match path_type {
                            PathTypes::Profiles => state.settings_handle.get_profile_directory().await,
                            PathTypes::MicProfiles => state.settings_handle.get_mic_profile_directory().await,
                            PathTypes::Presets => state.settings_handle.get_presets_directory().await,
                            PathTypes::Samples => state.settings_handle.get_samples_directory().await,
                            PathTypes::Icons => state.settings_handle.get_icons_directory().await,
                            PathTypes::Logs => state.settings_handle.get_log_directory().await,
                            PathTypes::Backups => state.settings_handle.get_backup_directory().await,
                        }) {
                            warn!("Error Opening Path: {:?}", error);
                        };
                    },
                    EventTriggers::OpenUi => {
                        if let Err(error) = open::that(get_util_url(&state)) {
                            warn!("Error Opening URL: {:?}", error);
                        }
                    },
                    EventTriggers::Activate => {
                        let activate = state.settings_handle.get_activate().await;
                        let url = get_util_url(&state);

                        // If saving window size is disabled, enforce default window dimensions (1271 x 770)
                        if !state.settings_handle.get_save_window_size().await {
                            if let Some(base_dirs) = directories::BaseDirs::new() {
                                let window_state_path = base_dirs.config_dir().join("com.frostycoolslug.goxlr-utility-ui").join(".window-state.json");
                                if window_state_path.exists() {
                                    if let Ok(content) = std::fs::read_to_string(&window_state_path) {
                                        if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
                                            if let Some(main) = json.get_mut("main") {
                                                main["width"] = serde_json::json!(1271);
                                                main["height"] = serde_json::json!(770);
                                                main["maximized"] = serde_json::json!(false);
                                                let _ = std::fs::write(&window_state_path, serde_json::to_string_pretty(&json).unwrap_or_default());
                                            }
                                        }
                                    }
                                } else {
                                    if let Some(parent) = window_state_path.parent() {
                                        let _ = std::fs::create_dir_all(parent);
                                    }
                                    let default_state = serde_json::json!({
                                        "main": {
                                            "width": 1271,
                                            "height": 770,
                                            "maximized": false,
                                            "visible": true,
                                            "decorated": true,
                                            "fullscreen": false
                                        }
                                    });
                                    let _ = std::fs::write(&window_state_path, serde_json::to_string_pretty(&default_state).unwrap_or_default());
                                }
                            }
                        }

                        // Use the temp directory as the runtime for any launched apps..
                        let tmp_dir = std::env::temp_dir();

                        #[cfg(not(unix))]
                        {
                            use windows_args;
                            match activate {
                                Some(exec) => {
                                    // Ok, we're going to force the app runtime into %TMP%, to
                                    // prevent situations where it may need to write files.


                                    let exec = exec.replace("%URL%", &url);
                                    let mut args = windows_args::Args::parse_cmd(&exec);
                                    if let Some(command) = args.next() {
                                        let result = Command::new(command)
                                            .current_dir(tmp_dir)
                                            .args(args)
                                            .stdout(Stdio::null())
                                            .stderr(Stdio::null())
                                            .spawn();

                                        if let Err(error) = result {
                                            warn!("Error Executing command: {:?}, falling back", error);
                                            if let Err(error) = open::that(url) {
                                                warn!("Error Opening URL: {:?}", error);
                                            }
                                        }
                                    }
                                },
                                None => {
                                    if let Err(error) = open::that(url) {
                                        warn!("Error Opening URL: {:?}", error);
                                    }
                                }
                            }

                            // After launching the UI, set its titlebar color to blend with the app background
                            std::thread::spawn(|| {
                                apply_titlebar_color();
                            });
                        }

                        #[cfg(unix)]
                        {
                            use shell_words;
                            match activate {
                                Some(exec) => {
                                    let exec = exec.replace("%URL%", &url);
                                    if let Ok(params) = shell_words::split(&exec) {
                                        debug!("Attempting to Execute: {:?}", params);
                                        let result = Command::new(&params[0])
                                            .current_dir(tmp_dir)
                                            .args(&params[1..])
                                            .stdout(Stdio::null())
                                            .stderr(Stdio::null())
                                            .spawn();

                                        if let Err(error) = result {
                                            warn!("Error Executing command: {:?}, falling back", error);
                                            if let Err(error) = open::that(url) {
                                                warn!("Error Opening URL: {:?}", error);
                                            }
                                        }

                                    } else if let Err(error) = open::that(url) {
                                        warn!("Error Opening URL: {:?}", error);
                                    }
                                },
                                None => {
                                    if let Err(error) = open::that(url) {
                                        warn!("Error Opening URL: {:?}", error);
                                    }
                                }
                            }
                        }

                    }
                }
            },
        }
    }
}

fn get_util_url(state: &DaemonState) -> String {
    let mut host = String::from("localhost");
    if state.http_settings.bind_address != "localhost"
        && &state.http_settings.bind_address != "0.0.0.0"
    {
        host.clone_from(&state.http_settings.bind_address);
    }

    format!("http://{}:{}/", host, state.http_settings.port)
}

/// Sets the titlebar color of the GoXLR Utility UI window to #0E0C1A using
/// the Windows DWM API (DWMWA_CAPTION_COLOR). Polls for the window to appear.
#[cfg(windows)]
fn apply_titlebar_color() {
    use std::thread::sleep;
    use std::time::Duration;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Dwm::{DWMWA_CAPTION_COLOR, DwmSetWindowAttribute};
    use windows::Win32::UI::WindowsAndMessaging::FindWindowW;
    use windows::core::w;

    // #0E0C1A as COLORREF (0x00BBGGRR)
    let color: u32 = 0x001A0C0E;

    // Wait for the UI window to appear (up to 10 seconds)
    for _ in 0..20 {
        sleep(Duration::from_millis(500));

        let hwnd = match unsafe { FindWindowW(None, w!("GoXLR Utility")) } {
            Ok(h) if h != HWND::default() => h,
            _ => continue,
        };

        let result = unsafe {
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_CAPTION_COLOR,
                &color as *const u32 as *const std::ffi::c_void,
                std::mem::size_of::<u32>() as u32,
            )
        };

        match result {
            Ok(()) => {
                debug!("Titlebar color set to #0E0C1A");
                return;
            }
            Err(e) => {
                warn!("Failed to set titlebar color: {:?}", e);
                return;
            }
        }
    }
    warn!("GoXLR Utility UI window not found for titlebar color");
}
