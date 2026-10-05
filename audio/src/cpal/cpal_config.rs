use anyhow::{Result, bail};
use cpal::Device;
use cpal::traits::{DeviceTrait, HostTrait};

pub struct CpalConfiguration {}

impl CpalConfiguration {
    pub(crate) fn get_device(device: Option<String>, input: bool) -> Result<Device> {
        let mut cpal_device = None;

        if let Some(device_name) = device
            && let Some(position) = device_name.find('*')
        {
            let str_host = &device_name[0..position];
            let str_device = &device_name[position + 1..device_name.len()];

            let cpal_host_list = cpal::available_hosts();
            let host_id = cpal_host_list.iter().find(|x| x.name() == str_host);

            if let Some(host_id) = host_id
                && let Ok(host) = cpal::host_from_id(*host_id)
            {
                cpal_device = if input {
                    if let Ok(mut devices) = host.input_devices() {
                        devices.find(|x| {
                            x.name().unwrap_or_else(|_| "UNKNOWN".to_string()) == str_device
                        })
                    } else {
                        None
                    }
                } else if let Ok(mut devices) = host.output_devices() {
                    devices
                        .find(|x| x.name().unwrap_or_else(|_| "UNKNOWN".to_string()) == str_device)
                } else {
                    None
                }
            };
        }

        if let Some(device) = cpal_device {
            Ok(device)
        } else {
            let host = cpal::default_host();
            let default_device = if input {
                host.default_input_device()
            } else {
                host.default_output_device()
            };

            match default_device {
                Some(device) => Ok(device),
                None => bail!("Unable to find Default Device"),
            }
        }
    }

    pub(crate) fn get_outputs() -> Vec<String> {
        let mut list: Vec<String> = vec![];

        let available_hosts = cpal::available_hosts();
        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id).unwrap();
            let devices = host.output_devices().unwrap();
            for device in devices {
                list.push(format!("{}*{}", host_id.name(), device.name().unwrap()));
            }
        }
        list
    }

    pub(crate) fn get_inputs() -> Vec<String> {
        let mut list: Vec<String> = vec![];

        let available_hosts = cpal::available_hosts();
        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id).unwrap();
            let devices = host.input_devices().unwrap();
            for device in devices {
                list.push(format!("{}*{}", host_id.name(), device.name().unwrap()));
            }
        }
        list
    }
}
