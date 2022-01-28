use color_eyre::Result;

use crate::prompt::prompt_select;
use crate::utils::{progress_callback, progress_callback2};
use clap::ArgMatches;
use core::dirs::get_dirs;
use core::op1::OP1;
use core::project::Project;
use indicatif::{ProgressBar, ProgressStyle};

// TODO: Get list of OP1 images in backup dir and pass to a select prompt
// TODO: Warn about overwrite, offer to save first
pub fn collect_args_and_run(arg_matches: Option<&ArgMatches>, mut op1: OP1) -> Result<()> {
    let project = prompt_select(
        Project::get_all_projects_in_dir(get_dirs().projects),
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
