use fruid::FromCLIInput;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput, Copy)]
struct ChannelMixSettings {
    level: u8, // 0-99
    pan: i8,   // Estimate -100 (all the way left) to 100 (all the way to the right)
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
struct EQSettings {
    low: u8,  // Estimate 0-100
    mid: u8,  // Estimate 0-100
    high: u8, // Estimate 0-100
}

// TODO: (maybe?) Create an enum of the effects with anonymous structs with better named fields
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
struct MasterEffectSettings {
    blue: u8,   // Estimate 0-100
    green: u8,  // Estimate 0-100
    white: u8,  // Estimate 0-100
    orange: u8, // Estimate 0-100
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
struct MasterOutSettings {
    left_balance: u8,  // 0-99
    right_balance: u8, // 0-99
    drive: u8,         //0-99
    release: u8,       // 0-300 (I think)
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
pub struct MixerSettings {
    per_channel_mix_settings: [ChannelMixSettings; 4],
    eq_settings: EQSettings,
    master_effect_settings: MasterEffectSettings,
    master_out_settings: MasterOutSettings,
}
