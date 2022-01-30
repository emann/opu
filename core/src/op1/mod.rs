pub mod dirs;
// pub mod subdirs;

use std::path::PathBuf;

use crate::dirs::get_dirs;
use crate::file_utils::{copy_dir_contents_with_progress, copy_items_with_progress};
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs, OP1Subdir};
use crate::project::Project;
use fs_extra::dir::{TransitProcess, TransitProcessResult};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::remove_dir_all;
use sysinfo::{DiskExt, SystemExt};

pub struct OP1 {
    pub op1_dirs: OP1Dirs,
    pub project: Option<Project>,
}

impl OP1 {
    pub fn find_connected_op1() -> Option<OP1> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        system
            .get_disks()
            .iter()
            .filter_map(|disk| OP1::try_from(disk.get_mount_point().to_path_buf()).ok())
            .next()
    }

    pub fn mount_point(&self) -> PathBuf {
        self.op1_dirs.parent_dir.clone()
    }

    // TODO: Handle errors
    // TODO: Only copy changed files
    /// Save project to device and to projects dir
    pub fn save_project<F>(&mut self, dirs_to_save: HashSet<OP1Subdir>, progress_handler: F)
    where
        F: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
    {
        match self.project.clone() {
            None => panic!("No project to save (eventually this will be an error to handle)"),
            Some(mut project) => {
                project.save();

                let project_dir = get_dirs().local_path_for_project(project);
                // TODO: Handle errors
                remove_dir_all(project_dir.clone());
                self.op1_dirs
                    .copy_to(project_dir, dirs_to_save, progress_handler);
            }
        }
    }

    // TODO: Some error handling
    pub fn load<F>(&self, project: Project, progress_handler: F)
    where
        F: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
    {
        remove_dir_all(self.mount_point());
        println!("removed!");

        let dirs: Vec<PathBuf> = project.op1_dirs.into_iter().collect();

        copy_items_with_progress(&dirs, self.mount_point(), progress_handler);
    }
}

impl From<OP1Dirs> for OP1 {
    fn from(op1_dirs: OP1Dirs) -> Self {
        // TODO: Better error handling in case of corruption
        Self {
            project: Project::try_from(&op1_dirs).ok(),
            op1_dirs,
        }
    }
}

impl TryFrom<PathBuf> for OP1 {
    type Error = OP1DirsError;

    fn try_from(mount_point: PathBuf) -> Result<Self, Self::Error> {
        OP1Dirs::try_from(mount_point).map(OP1::from)
    }
}
