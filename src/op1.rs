use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::slice::Iter;

use color_eyre::eyre::{ensure, eyre, Result};
use sysinfo::{DiskExt, ProcessExt, SystemExt};

const OP1_DIRECTORIES: [&'static str; 4] = ["album", "drum", "synth", "tape"];

pub(crate) struct Op1 {
    pub mount_point: PathBuf,
}

impl Op1 {
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

        Ok(Op1 {
            mount_point: mount_point.into(),
        })
    }

    pub(crate) fn find_connected_op1() -> Option<Op1> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        for disk in system.get_disks().iter() {
            if let Ok(op1) = Op1::from_mount_point(disk.get_mount_point()) {
                return Some(op1);
            }
        }
        None
    }

    pub(crate) fn subdirs(&self) -> Vec<PathBuf> {
        self.mount_point
            .read_dir()
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|dir| dir.path())
            .filter(|p| OP1_DIRECTORIES.contains(&p.file_name().unwrap().to_str().unwrap()))
            .collect()
    }
}
