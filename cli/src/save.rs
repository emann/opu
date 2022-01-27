use color_eyre::Result;

use crate::prompt::unwrap_or_prompt_input;
use crate::utils::progress_callback;
use clap::ArgMatches;
use core::metadata::{Metadata, MixerSettings, TempoSettings};
use core::op1::dirs::OP1Dirs;
use core::op1::OP1;
use core::project::Project;
use indicatif::{ProgressBar, ProgressStyle};

pub fn collect_args_and_run(arg_matches: Option<&ArgMatches>, mut op1: OP1) -> Result<()> {
    let name_arg = arg_matches.and_then(|am| am.value_of("name"));

    // Name unspecified and project metadata found on device - "Save" Operation, go right to saving
    if !(name_arg.is_none() && op1.project.is_some()) {
        // "Save As" Operation, get name if not supplied and create project
        let project_name = unwrap_or_prompt_input(
            arg_matches.and_then(|am| am.value_of("name")),
            "Project Name: ",
        )?;

        if op1.project.is_none() {
            let metadata = Metadata::new(
                project_name,
                TempoSettings::default(),
                MixerSettings::default(),
            );
            op1.project = Some(Project {
                metadata,
                op1_dirs: OP1Dirs::from(&op1),
            });
        }
    }

    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:.cyan/blue}] {bytes}/{total_bytes}"),
    );
    op1.save_project(progress_callback(pb));
    Ok(())
}
