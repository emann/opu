use num::traits::Bounded;
use opu_macros::FromCLIInput;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, FromCLIInput)]
pub struct TempoSettings {
    #[from_cli_input(prompt = "BPM")]
    bpm: f32,
    #[from_cli_input(prompt = "Tape Speed")]
    tape_speed: i8,
}

impl Eq for TempoSettings {}
