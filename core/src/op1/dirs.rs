use crate::file_utils::copy_items_with_progress;
use crate::op1::OP1;
use fs_extra::dir::TransitProcessResult;
use std::array::IntoIter;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use thiserror::Error;

use glob::{glob, GlobError};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parent directory is missing required directory \"{0}\"")]
    MissingDir(String),
}

#[derive(PartialEq, Eq, Hash)]
pub enum OP1Subdir {
    Album,
    Drum,
    Synth,
    Tape,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OP1Dirs {
    pub parent_dir: PathBuf,

    pub album: PathBuf,
    pub drum: PathBuf,
    pub synth: PathBuf,
    pub tape: PathBuf,
}

impl OP1Dirs {
    fn get_subdir_path(&self, subdir: OP1Subdir) -> PathBuf {
        match subdir {
            OP1Subdir::Album => self.album.clone(),
            OP1Subdir::Drum => self.drum.clone(),
            OP1Subdir::Synth => self.synth.clone(),
            OP1Subdir::Tape => self.tape.clone(),
        }
    }

    pub fn copy_to<P, F>(&self, dest_path: P, dirs_to_copy: HashSet<OP1Subdir>, progress_handler: F)
    where
        P: AsRef<Path> + Debug,
        F: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
    {
        let dirs_vec: Vec<PathBuf> = dirs_to_copy
            .into_iter()
            .map(|sd| self.get_subdir_path(sd))
            .collect();
        // TODO: Handle errors
        copy_items_with_progress(&dirs_vec, dest_path, progress_handler);
    }

    pub fn iter(&self) -> std::array::IntoIter<&Path, 4_usize> {
        [
            self.album.as_path(),
            self.drum.as_path(),
            self.synth.as_path(),
            self.tape.as_path(),
        ]
        .into_iter()
    }

    // TODO: Handle errors
    "Some more thought needs to go into how these will be stored/retrieved. Could create a .opu file\
    in the project dir that stores the metadata that goes into the opu_metadata.aiff file as well as\
    the hashes, would be good for when compression is a thing"
    pub fn get_hashes(&self) -> HashMap<PathBuf, u64> {
        let glob_str = self.parent_dir.join("/**/*.aiff");
        let f: Result<Vec<PathBuf>, GlobError> =
            glob(&glob_str.into_os_string().into_string().unwrap())
                .expect("Unable to glob")
                .into_iter()
                .collect();
        f.unwrap()
            .into_iter()
            .map(|d| {
                let relative_path = d.strip_prefix(&self.parent_dir).unwrap().to_owned();
                let hash = xxh3_64(&std::fs::read(&d).unwrap());
                (relative_path, hash)
            })
            .collect()
    }
}

impl TryFrom<PathBuf> for OP1Dirs {
    type Error = Error;

    // TODO: Handle/log IO errors when checking if dir exists
    fn try_from(parent_dir: PathBuf) -> Result<Self, Self::Error> {
        let album = parent_dir.join("album");
        match album.try_exists() {
            Ok(true) => (),
            _ => return Err(Error::MissingDir("album".to_owned())),
        }

        let drum = parent_dir.join("drum");
        match drum.try_exists() {
            Ok(true) => (),
            _ => return Err(Error::MissingDir("drum".to_owned())),
        }

        let synth = parent_dir.join("synth");
        match synth.try_exists() {
            Ok(true) => (),
            _ => return Err(Error::MissingDir("synth".to_owned())),
        }

        let tape = parent_dir.join("tape");
        match tape.try_exists() {
            Ok(true) => (),
            _ => return Err(Error::MissingDir("tape".to_owned())),
        }

        Ok(OP1Dirs {
            parent_dir,
            album,
            drum,
            synth,
            tape,
        })
    }
}

impl From<OP1> for OP1Dirs {
    fn from(op1: OP1) -> Self {
        op1.op1_dirs
    }
}

impl From<&OP1> for OP1Dirs {
    fn from(op1: &OP1) -> Self {
        op1.op1_dirs.clone()
    }
}

impl IntoIterator for OP1Dirs {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.album, self.drum, self.synth, self.tape].into_iter()
    }
}

// pub struct OP1DirsIterator {
//     op1_dirs: OP1Dirs,
//     index: usize,
// }
//
// impl Iterator for OP1DirsIterator {
//     type Item = PathBuf;
//
//     fn next(&mut self) -> Option<PathBuf> {
//         let result = match self.index {
//             0 => self.op1_dirs.album,
//             1 => self.op1_dirs.drum,
//             2 => self.op1_dirs.synth,
//             3 => self.op1_dirs.tape,
//             _ => return None,
//         };
//         self.index += 1;
//         Some(result)
//     }
// }
