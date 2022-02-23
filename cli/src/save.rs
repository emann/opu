use color_eyre::Result;

use crate::config::{Config, OPUConfig};
use crate::prompt::unwrap_or_prompt_input;
use crate::utils::progress_callback2;
use clap::ArgMatches;
use indicatif::{ProgressBar, ProgressStyle};
use opu_core::metadata::{Metadata, MixerSettings, TempoSettings};
use opu_core::op1::OP1;
use opu_core::project::Project;

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

    match op1.project() {
        Err(_) => {
            let project_name = unwrap_or_prompt_input(
                arg_matches.and_then(|am| am.value_of("name")),
                "Project Name: ",
            )?;
            let metadata = Metadata::new(
                project_name,
                TempoSettings::default(),
                MixerSettings::default(),
            );
            let local_path = config.local_path_for_project_name(&metadata.project_name);
            // Get path for local project with this name
            op1.save_as_new_project(metadata, local_path, progress_callback2(pb));
        }
        Ok(project) => {
            println!("Found project called {} on device", project.name());
            println!(
                "LP: {:?}",
                config.local_path_for_project_name(project.name())
            );
            op1.save_changed_files(
                // TODO: Handle errors
                config
                    .get_local_project(project.name())
                    .expect("No local project"),
                progress_callback2(pb),
            )
        }
    };

    Ok(())
}
