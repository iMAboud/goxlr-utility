use enum_map::Enum;
use strum::{Display, EnumIter, EnumProperty};

pub mod components;
pub mod error;
pub mod mic_profile;
pub mod microphone;
pub mod profile;

#[derive(Debug, Display, Enum, EnumIter, EnumProperty, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SampleButtons {
    #[strum(props(contextTitle = "sampleTopLeft"))]
    TopLeft,

    #[strum(props(contextTitle = "sampleTopRight"))]
    TopRight,

    #[strum(props(contextTitle = "sampleBottomLeft"))]
    BottomLeft,

    #[strum(props(contextTitle = "sampleBottomRight"))]
    BottomRight,

    #[strum(props(contextTitle = "sampleClear"))]
    Clear,
}

#[derive(Debug, EnumIter, Enum, EnumProperty, Copy, Clone, PartialEq)]
pub enum Preset {
    #[strum(props(tagSuffix = "preset1", contextTitle = "effects1"))]
    #[strum(to_string = "PRESET_1")]
    Preset1,

    #[strum(props(tagSuffix = "preset2", contextTitle = "effects2"))]
    #[strum(to_string = "PRESET_2")]
    Preset2,

    #[strum(props(tagSuffix = "preset3", contextTitle = "effects3"))]
    #[strum(to_string = "PRESET_3")]
    Preset3,

    #[strum(props(tagSuffix = "preset4", contextTitle = "effects4"))]
    #[strum(to_string = "PRESET_4")]
    Preset4,

    #[strum(props(tagSuffix = "preset5", contextTitle = "effects5"))]
    #[strum(to_string = "PRESET_5")]
    Preset5,

    #[strum(props(tagSuffix = "preset6", contextTitle = "effects6"))]
    #[strum(to_string = "PRESET_6")]
    Preset6,
}

#[derive(Debug, Enum, EnumIter, EnumProperty, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Faders {
    #[strum(props(
        faderContext = "FaderMeter0",
        muteContext = "mute1",
        scribbleContext = "scribble1"
    ))]
    A,

    #[strum(props(
        faderContext = "FaderMeter1",
        muteContext = "mute2",
        scribbleContext = "scribble2",
    ))]
    B,

    #[strum(props(
        faderContext = "FaderMeter2",
        muteContext = "mute3",
        scribbleContext = "scribble3",
    ))]
    C,

    #[strum(props(
        faderContext = "FaderMeter3",
        muteContext = "mute4",
        scribbleContext = "scribble4",
    ))]
    D,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_load_example_profiles() {
        let f1 = File::open("../profiles examples/MBdrGoxlr.goxlr").expect("MBdrGoxlr.goxlr should exist");
        let p1 = profile::Profile::load(f1);
        assert!(p1.is_ok(), "Failed to load MBdrGoxlr.goxlr: {:?}", p1.err());

        let f2 = File::open("../profiles examples/Sleep.goxlr").expect("Sleep.goxlr should exist");
        let p2 = profile::Profile::load(f2);
        assert!(p2.is_ok(), "Failed to load Sleep.goxlr: {:?}", p2.err());

        let f3 = File::open("../profiles examples/MBdrMics.goxlrMicProfile").expect("MBdrMics.goxlrMicProfile should exist");
        let p3 = mic_profile::MicProfileSettings::load(f3);
        assert!(p3.is_ok(), "Failed to load MBdrMics.goxlrMicProfile: {:?}", p3.err());
    }
}
