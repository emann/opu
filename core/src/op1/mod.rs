pub mod dirs;

use std::path::PathBuf;

use crate::file_utils::{copy_dir_contents_with_progress, copy_items_with_progress};
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs, OP1Subdir};
use crate::project::Project;
use fs_extra::dir::{TransitProcess, TransitProcessResult};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::remove_dir_all;
use std::{thread, time};
use sysinfo::{DiskExt, SystemExt};

use xxhash_rust::xxh3::xxh3_64;

pub struct OP1 {
    pub op1_dirs: OP1Dirs,
    pub project: Option<Project>,
}

impl OP1 {
    pub fn find_connected() -> Option<OP1> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        system
            .get_disks()
            .iter()
            .filter_map(|disk| OP1::try_from(disk.get_mount_point().to_path_buf()).ok())
            .next()
    }

    pub fn get_connected_op1_blocking() -> OP1 {
        let mut op1 = OP1::find_connected();
        while let None = op1 {
            thread::sleep(time::Duration::from_millis(100));
            op1 = OP1::find_connected();
        }
        op1.expect("Loop doesn't exit until op1 != None")
    }

    pub async fn get_connected_op1() -> OP1 {
        let mut op1 = OP1::find_connected();
        while let None = op1 {
            tokio::time::sleep(time::Duration::from_millis(100)).await;
            op1 = OP1::find_connected();
        }
        op1.expect("Loop doesn't exit until op1 != None")
    }

    pub fn mount_point(&self) -> PathBuf {
        self.op1_dirs.parent_dir.clone()
    }

    // TODO: Handle errors
    // TODO: Only copy changed files
    /// Save project to device and to projects dir
    pub fn save_project<F>(
        &mut self,
        dest: PathBuf,
        dirs_to_save: HashSet<OP1Subdir>,
        progress_handler: F,
    ) where
        F: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
    {
        match self.project.clone() {
            None => panic!("No project to save (eventually this will be an error to handle)"),
            Some(mut project) => {
                project.save();

                // TODO: Handle errors
                remove_dir_all(dest.clone());
                self.op1_dirs.copy_to(dest, dirs_to_save, progress_handler);
            }
        }
    }

    // TODO: Some error handling
    pub fn load<F>(&self, project: Project, progress_handler: F)
    where
        F: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
    {
        // TODO: Handle errors
        remove_dir_all(self.mount_point());
        println!("removed!");

        let dirs: Vec<PathBuf> = project.op1_dirs.into_iter().collect();
        // TODO: Handle errors
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
