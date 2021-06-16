use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::slice::Iter;

use color_eyre::eyre::{ensure, eyre, Result};
use sysinfo::{DiskExt, ProcessExt, SystemExt};

const OP1_DIRECTORIES: [&'static str; 4] = ["album", "drum", "synth", "tape"];

pub(crate) struct OP1Image {
    pub root_dir: PathBuf,
}

impl OP1Image {
    pub(crate) fn from_path(root_dir: &Path) -> Result<Self> {
        let child_dir_names: Vec<PathBuf> = root_dir
            .read_dir()?
            .filter_map(|d| d.ok())
            .map(|dir| dir.path())
            .collect();

        ensure!(
            OP1_DIRECTORIES
                .iter()
                .all(|&s| child_dir_names.contains(&root_dir.join(&s.to_string()))),
            "The directory provided does not contain all of the necessary child directories"
        );

        Ok(OP1Image {
            root_dir: root_dir.into(),
        })
    }

    pub(crate) fn find_connected_op1() -> Option<OP1Image> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        system
            .get_disks()
            .iter()
            .map(|disk| OP1Image::from_path(disk.get_mount_point()))
            .filter_map(|op1| op1.ok())
            .next()
    }

    pub(crate) fn subdirs(&self) -> Vec<PathBuf> {
        self.root_dir
            .read_dir()
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|dir| dir.path())
            .filter(|p| OP1_DIRECTORIES.contains(&p.file_name().unwrap().to_str().unwrap()))
            .collect()
    }
}
