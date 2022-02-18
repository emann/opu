use crate::project::Project;
use directories::ProjectDirs;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct CoreConfig {
    project_library_dir: PathBuf,
    patch_library_dir: PathBuf,
}

impl Default for CoreConfig {
    fn default() -> Self {
        let data_dir = CoreConfig::project_dirs().data_dir().to_path_buf();
        CoreConfig {
            project_library_dir: data_dir.join("projects"),
            patch_library_dir: data_dir.join("patch_library"),
        }
    }
}

impl AsRef<CoreConfig> for CoreConfig {
    fn as_ref(&self) -> &CoreConfig {
        &self
    }
}

impl OPUConfig for CoreConfig {}

// TODO: Make this a derive that requires a core: CoreConfig field
pub trait OPUConfig
where
    Self: DeserializeOwned + Serialize + Default + AsRef<CoreConfig>,
{
    fn project_dirs() -> ProjectDirs {
        let project_dirs =
            ProjectDirs::from("", "", "opu").expect("Unable to open projects directory");
        // TODO: Probably handle this
        std::fs::create_dir_all(project_dirs.config_dir())
            .expect("Should be able to create config dir");
        std::fs::create_dir_all(project_dirs.data_dir())
            .expect("Should be able to create data dir");
        project_dirs
    }

    fn path() -> PathBuf {
        Self::project_dirs().config_dir().join("config.toml")
    }

    fn load() -> ::std::io::Result<Self> {
        let config_file_path = Self::path();
        match config_file_path.try_exists() {
            Ok(true) => {
                let content = std::fs::read_to_string(Self::path())?;
                Ok(toml::from_str(&content)?)
            }
            Ok(false) => {
                let config: Self = Self::default();
                config.save()?;
                Ok(config)
            }
            Err(e) => Err(e),
        }
    }

    fn save(&self) -> ::std::io::Result<()> {
        let toml_str = toml::to_string(&self).unwrap();
        std::fs::write(Self::path(), toml_str)
    }

    fn local_path_for_project(&self, project: Project) -> PathBuf {
        let core_config: &CoreConfig = self.as_ref();
        return core_config
            .project_library_dir
            .join(&project.metadata.project_name);
    }

    fn project_library(&self) -> PathBuf {
        let core_config: &CoreConfig = self.as_ref();
        let project_library_dir = core_config.project_library_dir.clone();
        std::fs::create_dir_all(project_library_dir.clone())
            .expect("Should be able to create project dirs");
        project_library_dir
    }

    fn patch_library(&self) -> PathBuf {
        let core_config: &CoreConfig = self.as_ref();
        let patch_library_dir = core_config.patch_library_dir.clone();
        std::fs::create_dir_all(patch_library_dir.clone())
            .expect("Should be able to create project dirs");
        patch_library_dir
    }
}
