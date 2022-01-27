use core::mem::size_of;
use std::borrow::Cow;
use std::convert::TryInto;
use std::fs::read;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::__private::TryFrom;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::op1::dirs::OP1Dirs;
use crate::op1::OP1;
use crate::static_files::StaticFiles;
use std::ops::RangeInclusive;

const METADATA_FILENAME: &str = "opu_metadata.aif";
const METADATA_DIR: &str = "synth/_opu";
const SIZE_LABEL_BASE_ADDRESS: usize = 0x39490;
const SIZE_OF_SIZE_LABEL: usize = 8;
const METADATA_BASE_ADDRESS: usize = SIZE_LABEL_BASE_ADDRESS + SIZE_OF_SIZE_LABEL;
const SIZE_LABEL_BYTES_RANGE: RangeInclusive<usize> =
    SIZE_LABEL_BASE_ADDRESS..=(METADATA_BASE_ADDRESS - 1);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to find metadata file at {0}")]
    FileNotFound(PathBuf),
    #[error("Failed to read metadata file {0}")]
    FailedToRead(#[from] std::io::Error),
    #[error("Failed to read metadata file {0}")]
    FailedToDecodeUTF(#[from] std::str::Utf8Error),
    #[error("Failed to read metadata file {0}")]
    FailedToParseJSON(#[from] serde_json::Error),
}

#[derive(Default, Serialize, Deserialize, Clone)]
struct ChannelMixSettings {
    level: u8, // 0-99
    pan: i8,   // Estimate -100 (all the way left) to 100 (all the way to the right)
}

#[derive(Default, Serialize, Deserialize, Clone)]
struct EQSettings {
    low: u8,  // Estimate 0-100
    mid: u8,  // Estimate 0-100
    high: u8, // Estimate 0-100
}

// TODO: (maybe?) Create an enum of the effects with anonymous structs with better named fields
#[derive(Default, Serialize, Deserialize, Clone)]
struct MasterEffectSettings {
    blue: u8,   // Estimate 0-100
    green: u8,  // Estimate 0-100
    white: u8,  // Estimate 0-100
    orange: u8, // Estimate 0-100
}

#[derive(Default, Serialize, Deserialize, Clone)]
struct MasterOutSettings {
    left_balance: u8,  // 0-99
    right_balance: u8, // 0-99
    drive: u8,         //0-99
    release: u8,       // 0-300 (I think)
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct MixerSettings {
    per_channel_mix_settings: [ChannelMixSettings; 4],
    eq_settings: EQSettings,
    master_effect_settings: MasterEffectSettings,
    master_out_settings: MasterOutSettings,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct TempoSettings {
    bpm: f32,
    tape_speed: i8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub project_name: String,
    last_saved: DateTime<Local>,
    tempo_settings: TempoSettings,
    mixer_settings: MixerSettings,
    created: DateTime<Local>,
}

impl Metadata {
    pub fn new(
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

    // pub fn from_user_input() -> Metadata {
    //     // TODO: Collect tempo & mixer settings
    //     Metadata::new(
    //         prompt_input("Enter a name for the project: ").unwrap(),
    //         Default::default(),
    //         Default::default(),
    //     )
    // }

    // TODO: Use AsRef<Path>
    pub fn get_file_path(project_dir: PathBuf) -> PathBuf {
        return project_dir.join(METADATA_DIR).join(METADATA_FILENAME);
    }
}

impl TryFrom<PathBuf> for Metadata {
    type Error = Error;

    fn try_from(parent_dir: PathBuf) -> Result<Self, Self::Error> {
        let metadata_file = Metadata::get_file_path(parent_dir);
        metadata_file
            .try_exists()
            .map_err(|_| Error::FileNotFound(metadata_file.clone()))?;

        let file_bytes = read(metadata_file)?;

        let metadata_size = usize::from_be_bytes(
            file_bytes[SIZE_LABEL_BYTES_RANGE]
                .try_into()
                .expect("Should be able to get a usize"),
        );

        let metadata_str = std::str::from_utf8(
            &file_bytes[METADATA_BASE_ADDRESS..(METADATA_BASE_ADDRESS + metadata_size)],
        )?;
        let metadata: Metadata = serde_json::from_str(metadata_str)?;
        Ok(metadata)
    }
}

impl TryFrom<&OP1Dirs> for Metadata {
    type Error = Error;

    fn try_from(op1_dirs: &OP1Dirs) -> Result<Self, Self::Error> {
        let metadata_file = Metadata::get_file_path(op1_dirs.parent_dir.clone());
        Metadata::try_from(metadata_file)
    }
}

impl TryFrom<&OP1> for Metadata {
    type Error = Error;

    fn try_from(op1: &OP1) -> Result<Self, Self::Error> {
        Metadata::try_from(&OP1Dirs::from(op1))
    }
}

impl Into<Vec<u8>> for Metadata {
    fn into(self) -> Vec<u8> {
        let serialized_metadata = serde_json::to_string(&self).unwrap();
        let size_of_serialized_metadata = serialized_metadata.len();

        // Load the base metadata file into memory
        let mut metadata_file_bytes: Vec<u8> = match StaticFiles::get(METADATA_FILENAME).unwrap() {
            Cow::Borrowed(v) => v.to_owned(),
            Cow::Owned(v) => v,
        };

        // Inserting the size label
        metadata_file_bytes.splice(
            SIZE_LABEL_BASE_ADDRESS..(SIZE_LABEL_BASE_ADDRESS + size_of::<usize>()),
            // Only splice in the number of bytes required to store the size label to save space
            size_of_serialized_metadata.to_be_bytes().to_vec(),
        );

        // Inserting the metadata itself
        metadata_file_bytes.splice(
            METADATA_BASE_ADDRESS..(METADATA_BASE_ADDRESS + size_of_serialized_metadata),
            serialized_metadata.into_bytes(),
        );

        metadata_file_bytes
    }
}
