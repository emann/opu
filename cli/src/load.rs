use color_eyre::Result;

use crate::config::{Config, OPUConfig};
use crate::prompt::prompt_select;
use crate::utils::progress_callback2;
use clap::ArgMatches;
use indicatif::{ProgressBar, ProgressStyle};
use opu_core::op1::OP1;
use opu_core::project::Project;

// TODO: Get list of OP1 images in backup dir and pass to a select prompt
// TODO: Warn about overwrite, offer to save first
pub fn collect_args_and_run(
    config: Config,
    op1: OP1,
    _arg_matches: Option<&ArgMatches>,
) -> Result<()> {
    let project = prompt_select(
        Project::get_all_projects_in_dir(config.project_library()),
        "Select a project to load",
    )
    .unwrap();

    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:.cyan/blue}] {bytes}/{total_bytes}"),
    );
    op1.load(project, progress_callback2(pb));
    Ok(())
}
