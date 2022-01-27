use crate::metadata::{Error as MetadataError, Metadata};
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub struct Project {
    pub op1_dirs: OP1Dirs,
    pub metadata: Metadata,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Path \"{0}\" doesn't exist")]
    ParentDoesntExist(PathBuf),
    #[error("Path \"{0}\" doesn't exist")]
    OP1Dirs(#[from] OP1DirsError),
    #[error("Missing required dir: {0}")]
    Metadata(#[from] MetadataError),
}

impl Project {
    // pub fn get_all_projects_in_dir(path: PathBuf) -> Vec<Self> {
    //     path.read_dir()
    //         .unwrap()
    //         .filter_map(|d| d.ok())
    //         .map(|dir| Project::try_from(dir.path()))
    //         .filter_map(|p| p.ok())
    //         .collect()
    // }
    //
    pub fn save_to<T: AsRef<Path> + Debug>(&self, dest: T) {
        // TODO: Implement
        // Update time
        // If dest path != root path, save all
        // Else just save metadata
        println!("Project saved to {:?}", dest);
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.metadata.project_name)
    }
}

impl TryFrom<&OP1Dirs> for Project {
    type Error = MetadataError;

    fn try_from(op1_dirs: &OP1Dirs) -> Result<Self, Self::Error> {
        let metadata = Metadata::try_from(op1_dirs)?;
        Ok(Project {
            op1_dirs: op1_dirs.clone(),
            metadata,
        })
    }
}

impl TryFrom<PathBuf> for Project {
    type Error = Error;

    fn try_from(parent_dir: PathBuf) -> Result<Self, Self::Error> {
        if !parent_dir.exists() {
            return Err(Error::ParentDoesntExist(parent_dir));
        }

        let op1_dirs: OP1Dirs = OP1Dirs::try_from(parent_dir.clone())?;

        let metadata = Metadata::try_from(parent_dir.clone())?;

        Ok(Project { op1_dirs, metadata })
    }
}
