use std::env;
use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

use crate::file_utils::copy_dir_with_progress_bar;
use crate::op1::Op1;
use crate::prompt::{confirm, unwrap_or_prompt_input, prompt_input};
use crate::metadata::
use clap::ArgMatches;
use dialoguer::Confirm;

pub(crate) fn collect_args_and_run(arg_matches: Option<&ArgMatches>, op1: Op1) -> Result<()> {
    let project_name = unwrap_or_prompt_input(
        arg_matches.and_then(|am| am.value_of("name")),
        "Project Name: ",
    )?;

    // TODO: Collect tempo & mixer settings


    let dest = env::current_dir()?.join("temp").join(name);
    copy_dir_with_progress_bar(&op1.subdirs(), &dest)?;
    println!("Project saved to{:?}", dest);
    Ok(())
}
