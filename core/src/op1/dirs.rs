use crate::op1::OP1;
use std::convert::TryFrom;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parent directory is missing required directory \"{0}\"")]
    MissingDir(String),
}

#[derive(Clone)]
pub struct OP1Dirs {
    pub parent_dir: PathBuf,

    pub album: PathBuf,
    pub drum: PathBuf,
    pub synth: PathBuf,
    pub tape: PathBuf,
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
