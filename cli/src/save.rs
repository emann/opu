use color_eyre::Result;

use crate::config::{Config, OPUConfig};
use crate::prompt::unwrap_or_prompt_input;
use crate::utils::progress_callback2;
use clap::ArgMatches;
use indicatif::{ProgressBar, ProgressStyle};
use opu_core::metadata::{Metadata, MixerSettings, TempoSettings};
use opu_core::op1::dirs::{OP1Dirs, OP1Subdir};
use opu_core::op1::OP1;
use opu_core::project::Project;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn collect_args_and_run(
    config: Config,
    mut op1: OP1,
    arg_matches: Option<&ArgMatches>,
) -> Result<()> {
    let name_arg = arg_matches.and_then(|am| am.value_of("name"));

    // Name unspecified and project metadata found on device - "Save" Operation, go right to saving
    if !(name_arg.is_none() && op1.project.is_some()) {
        // "Save As" Operation, get name if not supplied and create project
        let project_name = unwrap_or_prompt_input(
            arg_matches.and_then(|am| am.value_of("name")),
            "Project Name: ",
        )?;

        if let None = op1.project {
            let metadata = Metadata::new(
                project_name,
                TempoSettings::default(),
                MixerSettings::default(),
            );
            op1.project = Some(Project {
                metadata,
                op1_dirs: OP1Dirs::from(&op1),
            });
        } else {
            op1.project.as_mut().unwrap().metadata.project_name = project_name;
        }
    }

    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:.cyan/blue}] {bytes}/{total_bytes}"),
    );

    let dirs_to_save = HashSet::from_iter(vec![
        OP1Subdir::Album,
        OP1Subdir::Drum,
        OP1Subdir::Synth,
        OP1Subdir::Tape,
    ]);

    op1.save_project(
        config.local_path_for_project(op1.project.clone().expect("Must have project by now")),
        dirs_to_save,
        progress_callback2(pb),
    );
    Ok(())
}
