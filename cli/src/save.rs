use std::env;
use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

use crate::prompt::{confirm, prompt_input, unwrap_or_prompt_input};
use clap::ArgMatches;
use core::dirs::get_projects_dir;
use core::file_utils::copy_items_with_progress_bar;
use core::metadata::Metadata;
use core::op1::OP1;
use core::project::Project;
use dialoguer::Confirm;

/*
1. Try to find metadata in op1 (TryFrom on Metadata path)
2. If there isn't metadata, create from user input + store on OP-1
3. Save to projects_dir/{project_name}
*/

// TODO: Warn about overwrite
pub fn collect_args_and_run(arg_matches: Option<&ArgMatches>, op1: OP1) -> Result<()> {
    // let project_name = unwrap_or_prompt_input(
    //     arg_matches.and_then(|am| am.value_of("name")),
    //     "Project Name: ",
    // )?;

    Project::from(op1).save();
    Ok(())
}
