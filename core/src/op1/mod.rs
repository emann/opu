pub mod dirs;
pub mod subdirs;

use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::slice::Iter;

use crate::metadata::Metadata;
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs};
use crate::project::Project;
use fs_extra::dir::remove;
use fs_extra::remove_items;
use std::convert::TryFrom;
use sysinfo::{DiskExt, ProcessExt, SystemExt};
use thiserror::Error;

pub struct OP1(OP1Dirs);

impl OP1 {
    fn project(&self) -> Option<Project> {
        Project::try_from(self).ok()
    }
    //
    // pub(crate) fn find_connected_op1() -> Option<OP1> {
    //     let mut system = sysinfo::System::new_all();
    //     system.refresh_all();
    //     for disk in system.get_disks().iter() {
    //         if let Ok(op1) = OP1::from_mount_point(disk.get_mount_point()) {
    //             return Some(op1);
    //         }
    //     }
    //     None
    // }

    // TODO: Some error handling
    // pub(crate) fn load(&self, project: Project) {
    //     remove_items(&self.subdirs());
    //     println!("removed!");
    //
    //     for project_subdir in project.subdirs() {
    //         copy_items_with_progress_bar(
    //             &project_subdir
    //                 .read_dir()?
    //                 .filter_map(|d| d.ok())
    //                 .map(|dir| dir.path())
    //                 .collect::<Vec<PathBuf>>(),
    //             self.mount_point
    //                 .join(project_subdir.file_name().unwrap().to_str().unwrap()),
    //         );
    //     }
    //     Ok(())
    // }
}

impl From<OP1Dirs> for OP1 {
    fn from(op1_dirs: OP1Dirs) -> Self {
        Self(op1_dirs)
    }
}

impl Into<OP1Dirs> for &OP1 {
    fn into(self) -> OP1Dirs {
        self.0.clone()
    }
}

impl TryFrom<PathBuf> for OP1 {
    type Error = OP1DirsError;

    fn try_from(parent_dir: PathBuf) -> Result<Self, Self::Error> {
        OP1Dirs::try_from(parent_dir).map(OP1::from)
    }
}

impl AsRef<OP1Dirs> for OP1 {
    fn as_ref(&self) -> &OP1Dirs {
        &self.0
    }
}
