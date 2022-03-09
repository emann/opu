use crate::op1::Effect;
use fruid::FromCLIInput;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput, Copy)]
pub struct ChannelMixSettings {
    pub level: u8, // 0-99
    pub pan: i8,   // Estimate -100 (all the way left) to 100 (all the way to the right)
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
pub struct EQSettings {
    pub low: u8,  // Estimate 0-100
    pub mid: u8,  // Estimate 0-100
    pub high: u8, // Estimate 0-100
}

// TODO: (maybe?) Create an enum of the effects with anonymous structs with better named fields
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
pub struct MasterEffectSettings {
    // pub effect: Effect,
    pub blue: u8,   // Estimate 0-100
    pub green: u8,  // Estimate 0-100
    pub white: u8,  // Estimate 0-100
    pub orange: u8, // Estimate 0-100
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
pub struct MasterOutSettings {
    pub left_balance: u8,  // 0-99
    pub right_balance: u8, // 0-99
    pub drive: u8,         //0-99
    pub release: u8,       // 0-300 (I think)
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, FromCLIInput)]
pub struct MixerSettings {
    pub per_channel_mix_settings: [ChannelMixSettings; 4],
    pub eq_settings: EQSettings,
    pub master_effect_settings: MasterEffectSettings,
    pub master_out_settings: MasterOutSettings,
}
