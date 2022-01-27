use std::convert::TryFrom;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parent directory is missing required dir \"{dir_name}\" {source}")]
    MissingDir {
        #[source]
        source: std::io::Error,
        dir_name: String,
    },
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

    fn try_from(parent_dir: PathBuf) -> Result<Self, Self::Error> {
        let album = parent_dir.join("album");
        // album.try_exists().map_err(|source| Error::MissingDir(source, name: "album"))?;
        let drum = parent_dir.join("drum");
        let synth = parent_dir.join("synth");
        let tape = parent_dir.join("tape");

        Ok(OP1Dirs {
            parent_dir,
            album,
            drum,
            synth,
            tape,
        })
    }
}
