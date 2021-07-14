use color_eyre::Result;
use platform_dirs::AppDirs;
use std::path::PathBuf;

//TODO: Allow this to be configurable e.g. save projects to desktop
pub(crate) fn get_projects_dir() -> PathBuf {
    let app_dirs = AppDirs::new(Some("opu"), true).expect("Unable to open projects directory");
    app_dirs.data_dir.join("projects")
}
