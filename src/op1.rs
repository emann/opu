use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::slice::Iter;

use crate::file_utils::copy_items_with_progress_bar;
use crate::metadata::Metadata;
use crate::project::Project;
use color_eyre::eyre::{ensure, eyre, Result};
use fs_extra::dir::remove;
use fs_extra::remove_items;
use sysinfo::{DiskExt, ProcessExt, SystemExt};

pub(crate) const OP1_DIRECTORIES: [&'static str; 4] = ["album", "drum", "synth", "tape"];

pub(crate) struct OP1 {
    pub mount_point: PathBuf,
}

impl OP1 {
    pub(crate) fn from_mount_point(mount_point: &Path) -> Result<Self> {
        let child_dir_names: Vec<PathBuf> = mount_point
            .read_dir()?
            .filter_map(|d| d.ok())
            .map(|dir| dir.path())
            .collect();

        ensure!(
            OP1_DIRECTORIES
                .iter()
                .all(|&s| child_dir_names.contains(&mount_point.join(&s.to_string()))),
            "The directory provided does not contain all of the necessary child directories"
        );

        Ok(OP1 {
            mount_point: mount_point.into(),
        })
    }

    pub(crate) fn find_connected_op1() -> Option<OP1> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        for disk in system.get_disks().iter() {
            if let Ok(op1) = OP1::from_mount_point(disk.get_mount_point()) {
                return Some(op1);
            }
        }
        None
    }

    // TODO: Some error handling
    pub(crate) fn load(&self, project: Project) -> Result<()> {
        // remove_items(&self.subdirs());
        // println!("removed!");

        for project_subdir in project.subdirs() {
            copy_items_with_progress_bar(
                &project_subdir
                    .read_dir()?
                    .filter_map(|d| d.ok())
                    .map(|dir| dir.path())
                    .collect::<Vec<PathBuf>>(),
                self.mount_point
                    .join(project_subdir.file_name().unwrap().to_str().unwrap()),
            );
        }
        Ok(())
    }
}

pub(crate) trait OP1Subdirs {
    // TODO: Probably could use a macro w/ an attribute instead of this
    fn get_root_dir(&self) -> PathBuf;

    fn subdirs(&self) -> Vec<PathBuf> {
        self.get_root_dir()
            .read_dir()
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|dir| dir.path())
            .filter(|p| OP1_DIRECTORIES.contains(&p.file_name().unwrap().to_str().unwrap()))
            .collect()
    }
}

impl OP1Subdirs for OP1 {
    fn get_root_dir(&self) -> PathBuf {
        self.mount_point.clone()
    }
}
