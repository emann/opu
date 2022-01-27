use crate::dirs::get_dirs;
use crate::metadata::{Error as MetadataError, Metadata};
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs};
use crate::op1::OP1;
use fs_extra::dir::create_all;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;

pub(crate) struct Project {
    pub(crate) op1_dirs: OP1Dirs,
    pub(crate) metadata: Metadata,
}

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Path \"{0}\" doesn't exist")]
    ParentDoesntExist(PathBuf),
    #[error("Path \"{0}\" doesn't exist")]
    OP1Dirs(#[from] OP1DirsError),
    #[error("Missing required dir: {0}")]
    Metadata(#[from] MetadataError),
}

impl Project {
    // pub(crate) fn get_all_projects_in_dir(path: PathBuf) -> Vec<Self> {
    //     path.read_dir()
    //         .unwrap()
    //         .filter_map(|d| d.ok())
    //         .map(|dir| Project::try_from(dir.path()))
    //         .filter_map(|p| p.ok())
    //         .collect()
    // }
    //
    // pub(crate) fn save(&self) {
    //     let metadata_file_bytes: Vec<u8> = self.metadata.clone().into();
    //     let path = Metadata::get_file_path(self.root_dir.clone());
    //     create_all(&path.parent().unwrap(), false);
    //
    //     File::create(path)
    //         .unwrap()
    //         .write_all(&metadata_file_bytes)
    //         .unwrap();
    //
    //     let dest = get_dirs().join(self.metadata.project_name.clone());
    //
    //     // copy_items_with_progress_bar(&self.subdirs(), &dest);
    //     println!("Project saved to {:?}", dest);
    // }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.metadata.project_name)
    }
}

// TODO: impl TryInto<Project> for AsRef<OP1Dirs> or something similar
impl TryFrom<&OP1> for Project {
    type Error = MetadataError;

    fn try_from(op1: &OP1) -> Result<Self, Self::Error> {
        let op1_dirs: OP1Dirs = op1.into();
        Project::try_from(op1_dirs)
    }
}

impl TryFrom<OP1Dirs> for Project {
    type Error = MetadataError;

    fn try_from(op1_dirs: OP1Dirs) -> Result<Self, Self::Error> {
        let metadata = Metadata::try_from(&op1_dirs)?;
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
