use crate::op1::OP1Directories;
use chrono::serde as chrono_serde;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

const METADATA_START_ADDRESS: u32 = 0x3c400;

#[derive(Default, Serialize, Deserialize)]
struct ChannelMixSettings {
    level: u8, // 0-99
    pan: i8,   // Estimate -100 (all the way left) to 100 (all the way to the right)
}

#[derive(Default, Serialize, Deserialize)]
struct EQSettings {
    low: u8,  // Estimate 0-100
    mid: u8,  // Estimate 0-100
    high: u8, // Estimate 0-100
}

// TODO: (maybe?) Create an enum of the effects with anonymous structs with better named fields
#[derive(Default, Serialize, Deserialize)]
struct MasterEffectSettings {
    blue: u8,   // Estimate 0-100
    green: u8,  // Estimate 0-100
    white: u8,  // Estimate 0-100
    orange: u8, // Estimate 0-100
}

#[derive(Default, Serialize, Deserialize)]
struct MasterOutSettings {
    left_balance: u8,  // 0-99
    right_balance: u8, // 0-99
    drive: u8,         //0-99
    release: u8,       // 0-300 (I think)
}

#[derive(Default, Serialize, Deserialize)]
struct MixerSettings {
    per_channel_mix_settings: [ChannelMixSettings; 4],
    eq_settings: EQSettings,
    master_effect_settings: MasterEffectSettings,
    master_out_settings: MasterOutSettings,
}

#[derive(Default, Serialize, Deserialize)]
struct TempoSettings {
    bpm: f32,
    tape_speed: i8,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    project_name: String,
    created: DateTime<Local>,
    last_saved: DateTime<Local>,
    tempo_settings: TempoSettings,
    mixer_settings: MixerSettings,
}

impl Metadata {
    fn new(
        project_name: String,
        tempo_settings: TempoSettings,
        mixer_settings: MixerSettings,
    ) -> Metadata {
        let now = Local::now();
        Metadata {
            project_name,
            created: now,
            last_saved: now,
            tempo_settings,
            mixer_settings,
        }
    }
}
