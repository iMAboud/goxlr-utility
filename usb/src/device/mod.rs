use crate::device::base::AttachGoXLR;
use crate::device::base::FullGoXLRDevice;
use crate::device::base::GoXLRDevice;
use anyhow::Result;
use goxlr_types::{DriverInterface, VersionNumber};
use tokio::sync::mpsc::Sender;

pub mod base;

mod tusb;
use crate::device::tusb::device;

pub fn get_version() -> (DriverInterface, Option<VersionNumber>) {
    device::get_interface_version()
}

pub fn find_devices() -> Vec<GoXLRDevice> {
    device::find_devices()
}

pub fn from_device(
    device: GoXLRDevice,
    disconnect_sender: Sender<String>,
    event_sender: Sender<String>,
    skip_pause: bool,
) -> Result<Box<dyn FullGoXLRDevice>> {
    device::TUSBAudioGoXLR::from_device(device, disconnect_sender, event_sender, skip_pause)
}
