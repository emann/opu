use crate::project::Project;
use platform_dirs::AppDirs;
use std::path::PathBuf;

pub struct Dirs {
    pub projects: PathBuf,
    pub patch_library: PathBuf,
}

impl Dirs {
    pub fn local_path_for_project(&self, project: Project) -> PathBuf {
        self.projects.join(&project.metadata.project_name)
    }
}

//TODO: Allow this to be configurable e.g. save projects to desktop
pub fn get_dirs() -> Dirs {
    let app_dirs = AppDirs::new(Some("opu"), true).expect("Unable to open projects directory");

    return Dirs {
        projects: app_dirs.data_dir.join("projects"),
        patch_library: app_dirs.data_dir.join("patch_library"),
    };
}
