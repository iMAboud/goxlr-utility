mod windows;

use crate::DaemonState;
use crate::events::EventTriggers;
use anyhow::Result;
use tokio::sync::mpsc;

pub fn handle_tray(state: DaemonState, tx: mpsc::Sender<EventTriggers>) -> Result<()> {
    windows::handle_tray(state, tx)
}
