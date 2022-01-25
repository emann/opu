use crate::dirs::get_projects_dir;
use crate::file_utils::copy_items_with_progress_bar;
use crate::metadata::{Metadata, MetadataError};
use crate::op1::{OP1Subdirs, OP1, OP1_DIRECTORIES};
use fs_extra::dir::create_all;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) struct Project {
    pub(crate) root_dir: PathBuf,
    pub(crate) metadata: Metadata,
}

impl Project {
    pub(crate) fn get_all_projects_in_dir(path: PathBuf) -> Vec<Self> {
        path.read_dir()
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|dir| Project::try_from(dir.path()))
            .filter_map(|p| p.ok())
            .collect()
    }

    pub(crate) fn save(&self) {
        let metadata_file_bytes: Vec<u8> = self.metadata.clone().into();
        let path = Metadata::get_file_path(self.root_dir.clone());
        create_all(&path.parent().unwrap(), false);

        File::create(path)
            .unwrap()
            .write_all(&metadata_file_bytes)
            .unwrap();

        let dest = get_projects_dir().join(self.metadata.project_name.clone());

        copy_items_with_progress_bar(&self.subdirs(), &dest);
        println!("Project saved to {:?}", dest);
    }
}

impl OP1Subdirs for Project {
    fn get_root_dir(&self) -> PathBuf {
        self.root_dir.clone()
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.metadata.project_name)
    }
}

impl TryFrom<PathBuf> for Project {
    type Error = Report;

    fn try_from(mut path: PathBuf) -> Result<Self, Self::Error> {
        if !path.exists() {
            // return Err(eyre!("Path doesn't exist: {:?}", path));
        }

        let metadata = Metadata::try_from(Metadata::get_file_path(path.clone()))?;

        Ok(Project {
            root_dir: path,
            metadata,
        })
    }
}

// TODO: Metadata::try_from(path) should return better errors, this should check specifically for the metadata file missing
impl From<OP1> for Project {
    fn from(op1: OP1) -> Self {
        Project::try_from(op1.mount_point.clone()).unwrap_or_else(|_| Project {
            root_dir: op1.mount_point,
            metadata: Metadata::from_user_input(),
        })
    }
}
