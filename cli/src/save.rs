use color_eyre::Result;

use crate::config::{Config, OPUConfig};
use crate::prompt::unwrap_or_prompt_input;
use crate::utils::progress_callback2;
use clap::ArgMatches;
use indicatif::{ProgressBar, ProgressStyle};
use opu_core::metadata::Metadata;
use opu_core::op1::OP1;

pub fn collect_args_and_run(
    config: Config,
    mut op1: OP1,
    arg_matches: Option<&ArgMatches>,
) -> Result<()> {
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:.cyan/blue}] {bytes}/{total_bytes}"),
    );

    let local_path =
        config.local_path_for_project_name(&op1.project().unwrap().metadata.project_name);
    // Get path for local project with this name
    op1.save_project(local_path, progress_callback2(pb));

    Ok(())
}
