use std::convert::TryFrom;
use std::env;
use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

use crate::prompt::{confirm, prompt_input, unwrap_or_prompt_input};
use clap::ArgMatches;
use core::metadata::{Metadata, MixerSettings, TempoSettings};
use core::op1::dirs::OP1Dirs;
use core::op1::OP1;
use core::project::Project;
use dialoguer::Confirm;

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

    op1.save_project();
    Ok(())
}
