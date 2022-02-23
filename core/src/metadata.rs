use core::mem::size_of;
use std::convert::{TryFrom, TryInto};
use std::fs::{read, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use memchr::memmem;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::op1::dirs::OP1Dirs;
use crate::op1::OP1;
use fs_extra::dir::create_all;
use include_flate::flate;

flate!(static BASE_METADATA_FILE_BYTES: [u8] from "assets/opu_metadata.aif");

const METADATA_FILENAME: &str = "opu_metadata.aif";
const METADATA_DIR: &str = "synth/_opu";

const COOKIE: &[u8; 4] = b"OPU:";
const COOKIE_SIZE: usize = 4;
const SIZE_OF_SIZE_LABEL: usize = size_of::<usize>();

// These are the addresses used when *writing* the metadata file - theres no guarantee that the OP-1
// won't alter the file slightly
const COOKIE_BASE_ADDRESS: usize = 0x39490;
const SIZE_LABEL_BASE_ADDRESS: usize = COOKIE_BASE_ADDRESS + COOKIE_SIZE;
const METADATA_BASE_ADDRESS: usize = SIZE_LABEL_BASE_ADDRESS + SIZE_OF_SIZE_LABEL;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to find metadata file at {0}")]
    FileNotFound(PathBuf),
    #[error("Failed to read metadata file {0}")]
    FailedToRead(#[from] std::io::Error),
    #[error("Unable to find cookie in metadata file")]
    CookieNotFound,
    #[error("Failed to read metadata file {0}")]
    FailedToDecodeUTF(#[from] std::str::Utf8Error),
    #[error("Failed to read metadata file {0}")]
    FailedToParseJSON(#[from] serde_json::Error),
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct ChannelMixSettings {
    level: u8, // 0-99
    pan: i8,   // Estimate -100 (all the way left) to 100 (all the way to the right)
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct EQSettings {
    low: u8,  // Estimate 0-100
    mid: u8,  // Estimate 0-100
    high: u8, // Estimate 0-100
}

// TODO: (maybe?) Create an enum of the effects with anonymous structs with better named fields
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct MasterEffectSettings {
    blue: u8,   // Estimate 0-100
    green: u8,  // Estimate 0-100
    white: u8,  // Estimate 0-100
    orange: u8, // Estimate 0-100
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct MasterOutSettings {
    left_balance: u8,  // 0-99
    right_balance: u8, // 0-99
    drive: u8,         //0-99
    release: u8,       // 0-300 (I think)
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MixerSettings {
    per_channel_mix_settings: [ChannelMixSettings; 4],
    eq_settings: EQSettings,
    master_effect_settings: MasterEffectSettings,
    master_out_settings: MasterOutSettings,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TempoSettings {
    bpm: f32,
    tape_speed: i8,
}

impl Eq for TempoSettings {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub project_name: String,
    pub last_saved: DateTime<Local>,
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

    pub fn get_file_path<P>(project_parent_dir: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        return project_parent_dir
            .as_ref()
            .to_path_buf()
            .join(METADATA_DIR)
            .join(METADATA_FILENAME);
    }

    pub fn save<P>(&mut self, parent_dir: P)
    where
        P: AsRef<Path>,
    {
        self.last_saved = Local::now();
        let metadata_file_bytes: Vec<u8> = self.clone().into();

        let path = Metadata::get_file_path(parent_dir);
        // TODO: Handle errors
        create_all(path.parent().expect("Parent must exist"), true);
        println!("{:?}", path);
        File::create(path)
            .unwrap()
            .write_all(&metadata_file_bytes)
            .unwrap();
    }
}

impl AsRef<Metadata> for Metadata {
    fn as_ref(&self) -> &Metadata {
        &self
    }
}

impl TryFrom<PathBuf> for Metadata {
    type Error = Error;

    fn try_from(metadata_file: PathBuf) -> Result<Self, Self::Error> {
        metadata_file
            .try_exists()
            .map_err(|_| Error::FileNotFound(metadata_file.clone()))?;

        let file_bytes = read(metadata_file)?;

        let cookie_start_address =
            memmem::find(&file_bytes, COOKIE).ok_or(Error::CookieNotFound)?;
        let size_label_base_address = cookie_start_address + COOKIE_SIZE;
        let metadata_base_address = size_label_base_address + SIZE_OF_SIZE_LABEL;

        let metadata_size = usize::from_be_bytes(
            file_bytes[size_label_base_address..metadata_base_address]
                .try_into()
                .expect("Should be able to get a usize"),
        );

        let metadata_str = std::str::from_utf8(
            &file_bytes[metadata_base_address..(metadata_base_address + metadata_size)],
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
        let mut metadata_file_bytes: Vec<u8> = BASE_METADATA_FILE_BYTES.to_vec();

        // Inserting the cookie
        metadata_file_bytes.splice(
            COOKIE_BASE_ADDRESS..(COOKIE_BASE_ADDRESS + COOKIE_SIZE),
            // Only splice in the number of bytes required to store the size label to save space
            COOKIE.to_vec(),
        );

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
