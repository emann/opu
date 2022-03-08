use std::convert::TryFrom;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::{thread, time};

use fs_extra::dir::TransitProcessResult;
use sysinfo::{DiskExt, SystemExt};

use crate::file_utils::copy_items_with_progress;
use crate::metadata::Error as MetadataError;
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs};
use crate::project::Project;

pub mod dirs;

pub struct OP1 {
    pub op1_dirs: OP1Dirs,
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
        #[cfg(debug_assertions)]
        {
            use std::env;
            if let Ok(path_str) = env::var("DUMMY_OP1_PATH") {
                let path: PathBuf = path_str.into();
                println!("Creating/using dummy OP-1 at {:?}", path);
                create_dir_all(path.join("album"));
                create_dir_all(path.join("drum"));
                create_dir_all(path.join("synth"));
                create_dir_all(path.join("tape"));
                return OP1::try_from(path)
                    .expect("The value of the DUMMY_OP1_PATH must be a valid path");
            }
        }

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

    pub fn project(&self) -> Result<Project, MetadataError> {
        Project::try_from(&self.op1_dirs)
    }

    // TODO: Handle errors
    /// Save project to device and to projects dir
    pub fn save_project<P>(&mut self, dest: PathBuf, progress_handler: P)
    where
        P: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
    {
        let mut project = self.project().expect("Project must be available");
        project.save_metadata();
        create_dir_all(&dest).expect("Failed to create dir");

        let dirs: Vec<&Path> = self.op1_dirs.iter().collect();
        copy_items_with_progress(&dirs, dest, progress_handler).expect("Failed to copy items");
    }

    // TODO: Some error handling
    pub fn load<F>(&self, project: Project, progress_handler: F)
    where
        F: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
    {
        // TODO: Handle errors
        remove_dir_all(self.mount_point()).expect("Failed to remove dirs");
        println!("removed!");

        let dirs: Vec<PathBuf> = project.op1_dirs.into_iter().collect();
        // TODO: Handle errors
        copy_items_with_progress(&dirs, self.mount_point(), progress_handler)
            .expect("Failed to copy items");
    }
}

impl From<OP1Dirs> for OP1 {
    fn from(op1_dirs: OP1Dirs) -> Self {
        // TODO: Better error handling in case of corruption
        Self { op1_dirs }
    }
}

impl TryFrom<PathBuf> for OP1 {
    type Error = OP1DirsError;

    fn try_from(mount_point: PathBuf) -> Result<Self, Self::Error> {
        OP1Dirs::try_from(mount_point).map(OP1::from)
    }
}
