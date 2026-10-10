mod windows;

use crate::DaemonState;
use crate::events::EventTriggers;
use anyhow::Result;
use std::path::PathBuf;
use tokio::sync::mpsc;
use which::which;

pub fn perform_preflight() -> Result<()> {
    windows::perform_platform_preflight()
}

pub async fn spawn_runtime(state: DaemonState, tx: mpsc::Sender<EventTriggers>) -> Result<()> {
    windows::spawn_platform_runtime(state, tx).await
}

pub fn has_autostart() -> bool {
    windows::has_autostart()
}

pub fn set_autostart(enabled: bool) -> Result<()> {
    if enabled {
        return windows::create_startup_link();
    }
    windows::remove_startup_link()
}

pub fn display_error(message: String) {
    windows::display_error(message);
}

pub fn get_ui_app_path() -> Option<PathBuf> {
    let mut path = None;
    let bin_name = get_ui_binary_name();

    let cwd = std::env::current_dir().unwrap().join(bin_name.clone());
    if cwd.exists() {
        path.replace(cwd);
    }

    if path.is_none()
        && let Some(parent) = std::env::current_exe().unwrap().parent()
    {
        let bin = parent.join(bin_name.clone());
        if bin.exists() {
            path.replace(bin);
        }
    }

    if path.is_none()
        && let Ok(which) = which(bin_name)
    {
        path.replace(which);
    }

    path
}

static UI_NAME: &str = "goxlr-utility-ui";
fn get_ui_binary_name() -> String {
    format!("{UI_NAME}.exe")
}
