use std::path::{Path, PathBuf};

pub struct PatchLibrary {
    root_dir: Path,
}

pub struct PatchLibraryChildren {
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

pub struct PatchPack {
    path: PathBuf,
    name: String,
}

impl PatchLibrary {
    fn children(&self) -> std::io::Result<PatchLibraryChildren> {
        let (dirs, files): (Vec<PathBuf>, Vec<PathBuf>) = self
            .root_dir
            .read_dir()?
            .into_iter()
            .filter_map(|d| match d {
                Ok(dir_entry) => Some(dir_entry.path()),
                Err(_) => None,
            })
            .partition(|d| d.is_dir());
        Ok(PatchLibraryChildren { dirs, files })
    }

    pub fn drum_patches(&self) -> Vec<PathBuf> {
        // Go through each dir in PLC, see if theres a drums dir, make a PP from it
        //
    }
}
