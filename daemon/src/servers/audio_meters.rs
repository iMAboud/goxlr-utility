use std::collections::HashMap;

#[cfg(windows)]
pub fn get_endpoint_peak_levels() -> HashMap<String, f32> {
    use std::collections::HashMap;
    use windows::Win32::Foundation::PROPERTYKEY;
    use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
    use windows::Win32::Media::Audio::{
        DEVICE_STATE_ACTIVE, IMMDeviceEnumerator, MMDeviceEnumerator, eAll,
    };
    use windows::Win32::System::Com::{
        CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, STGM,
    };
    use windows::core::{GUID, Result};

    let mut levels = HashMap::new();

    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let enumerator: Result<IMMDeviceEnumerator> =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL);
        let Ok(enumerator) = enumerator else {
            return levels;
        };

        let Ok(collection) = enumerator.EnumAudioEndpoints(eAll, DEVICE_STATE_ACTIVE) else {
            return levels;
        };

        let Ok(count) = collection.GetCount() else {
            return levels;
        };

        // PKEY_Device_FriendlyName: {A45C254E-DF1C-4EFD-8020-67D146A850E0}, pid: 14
        let pkey_friendly_name = PROPERTYKEY {
            fmtid: GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
            pid: 14,
        };

        for i in 0..count {
            let Ok(device) = collection.Item(i) else {
                continue;
            };

            let Ok(store) = device.OpenPropertyStore(STGM(0)) else {
                continue;
            };

            let Ok(prop) = store.GetValue(&pkey_friendly_name) else {
                continue;
            };

            let pwstr = prop.Anonymous.Anonymous.Anonymous.pwszVal;
            let Ok(name_str) = pwstr.to_string() else {
                continue;
            };

            if name_str.is_empty() {
                continue;
            }

            let Ok(meter) = device.Activate::<IAudioMeterInformation>(CLSCTX_ALL, None) else {
                continue;
            };

            if let Ok(peak) = meter.GetPeakValue() {
                let name_lower = name_str.to_lowercase();
                let channel_key = if name_lower.contains("music") {
                    "Music"
                } else if name_lower.contains("chat mic")
                    || (name_lower.contains("mic") && !name_lower.contains("monitor"))
                {
                    "Mic"
                } else if name_lower.contains("chat") {
                    "Chat"
                } else if name_lower.contains("game") {
                    "Game"
                } else if name_lower.contains("system") {
                    "System"
                } else if name_lower.contains("sample") {
                    "Sample"
                } else if name_lower.contains("line in") || name_lower.contains("linein") {
                    "LineIn"
                } else if name_lower.contains("console") {
                    "Console"
                } else if name_lower.contains("headphones") {
                    "Headphones"
                } else if name_lower.contains("line out") || name_lower.contains("lineout") {
                    "LineOut"
                } else {
                    ""
                };

                if !channel_key.is_empty() {
                    levels.insert(channel_key.to_string(), peak);
                }
                levels.insert(name_str, peak);
            }
        }
    }

    levels
}

#[cfg(not(windows))]
pub fn get_endpoint_peak_levels() -> HashMap<String, f32> {
    HashMap::new()
}
