use crate::metadata::{Error as MetadataError, Metadata};
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs};
use glob::{glob, GlobError};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::path::PathBuf;
use thiserror::Error;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub op1_dirs: OP1Dirs,
    pub metadata: Metadata,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Path \"{0}\" doesn't exist")]
    ParentDoesntExist(PathBuf),
    #[error("Path \"{0}\" doesn't exist")]
    OP1Dirs(#[from] OP1DirsError),
    #[error("Missing required dir: {0}")]
    Metadata(#[from] MetadataError),
}

impl Project {
    pub fn new(op1_dirs: OP1Dirs, metadata: Metadata) -> Project {
        Project { op1_dirs, metadata }
    }

    pub fn get_all_projects_in_dir(path: PathBuf) -> Vec<Self> {
        path.read_dir()
            .unwrap()
            .filter_map(|d| d.ok())
            .map(|dir| Project::try_from(dir.path()))
            .filter_map(|p| p.ok())
            .collect()
    }

    pub fn root_dir(&self) -> PathBuf {
        self.op1_dirs.parent_dir.clone()
    }

    pub fn save_metadata(&mut self) {
        self.metadata.save(&self.op1_dirs.parent_dir)
    }

    // TODO: Handle errors
    // Some more thought needs to go into how these will be stored/retrieved. Could create a .opu file
    // in the project dir that stores the metadata that goes into the opu_metadata.aiff file as well as
    // the hashes, would be good for when compression is a thing
    //
    // This should do the hashing in a thread so that it can be awaited onx
    fn get_hashes(&self) -> HashMap<PathBuf, u64> {
        let glob_str = self.root_dir().join("**/*.aif");

        let glob_matches: Result<Vec<PathBuf>, GlobError> =
            glob(&glob_str.into_os_string().into_string().unwrap())
                .expect("Unable to glob")
                .into_iter()
                .collect();
        glob_matches
            .expect("Got a glob error")
            .into_iter()
            .map(|d| {
                let relative_path = d.strip_prefix(&self.root_dir()).unwrap().to_owned();
                let hash = xxh3_64(&std::fs::read(&d).unwrap());
                (relative_path, hash)
            })
            .collect()
    }

    pub fn get_changed_files(&self, other_project: &Project) -> Vec<PathBuf> {
        use std::time::Instant;
        let mut now = Instant::now();
        println!("Getting my hashes");
        let my_hashes = self.get_hashes();
        println!("Done {:.2?}", now.elapsed());

        now = Instant::now();
        println!("Getting other hashes");
        let other_hashes = other_project.get_hashes();
        println!("Done {:.2?}", now.elapsed());
        println!("Comparing hashes");
        my_hashes
            .into_iter()
            .filter_map(|(relative_path, hash)| {
                if other_hashes.get(&relative_path)? != &hash {
                    // Relative path found and hash is different, this file has changed
                    Some(self.root_dir().join(relative_path))
                } else {
                    // Relative path found and hash is identical, no changes
                    None
                }
            })
            .collect()
    }

    pub fn name(&self) -> &str {
        &self.metadata.project_name
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.metadata.project_name)
    }
}

impl TryFrom<&OP1Dirs> for Project {
    type Error = MetadataError;

    fn try_from(op1_dirs: &OP1Dirs) -> Result<Self, Self::Error> {
        let metadata = Metadata::try_from(op1_dirs)?;
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

        let metadata = Metadata::try_from(&op1_dirs)?;

        Ok(Project { op1_dirs, metadata })
    }
}

impl AsRef<Metadata> for Project {
    fn as_ref(&self) -> &Metadata {
        &self.metadata
    }
}
