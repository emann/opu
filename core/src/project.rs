use crate::metadata::{Error as MetadataError, Metadata};
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn new(op1_dirs: OP1Dirs, metadata: Metadata) -> Project {
        Project { op1_dirs, metadata }
    }

    pub fn get_all_projects_in_dir(path: PathBuf) -> Vec<Self> {
        path.read_dir()
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|dir| Project::try_from(dir.path()))
            .filter_map(|p| p.ok())
            .collect()
    }

    pub fn root_dir(&self) -> PathBuf {
        self.op1_dirs.parent_dir.clone()
    }

    pub fn save_metadata(&mut self) {
        self.metadata.save(&self.op1_dirs.parent_dir)
    }

    pub fn name(&self) -> &str {
        &self.metadata.project_name
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

        let metadata = Metadata::try_from(&op1_dirs)?;

        Ok(Project { op1_dirs, metadata })
    }
}

impl AsRef<Metadata> for Project {
    fn as_ref(&self) -> &Metadata {
        &self.metadata
    }
}
