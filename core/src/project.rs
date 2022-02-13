use crate::dirs::get_dirs;
use crate::metadata::{Error as MetadataError, Metadata};
use crate::op1::dirs::{Error as OP1DirsError, OP1Dirs};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectLibrary {
    pub projects: Vec<Project>,
}

impl Default for ProjectLibrary {
    fn default() -> Self {
        let projects = Project::get_all_projects_in_dir(get_dirs().projects);
        Self { projects }
    }
}

impl From<ProjectLibrary> for Vec<Project> {
    fn from(pl: ProjectLibrary) -> Self {
        pl.projects
    }
}

impl IntoIterator for ProjectLibrary {
    type Item = Project;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.projects.into_iter()
    }
}

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

    pub fn save(&mut self) {
        self.metadata.save(&self.op1_dirs.parent_dir)
    }
}

impl Default for Project {
    fn default() -> Self {
        ProjectLibrary::default()
            .into_iter()
            .next()
            .expect("For now we assume there are project available locally")
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
