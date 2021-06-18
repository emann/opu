use std::env;
use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

use crate::file_utils::copy_dir_with_progress_bar;
use crate::op1::Op1;
use crate::prompt::unwrap_or_prompt_input;
use clap::ArgMatches;

// TODO: Get list of OP1 images in backup dir and pass to a select prompt
pub(crate) fn collect_args_and_run(arg_matches: Option<&ArgMatches>, op1: Op1) -> Result<()> {
    let backup_name = unwrap_or_prompt_input(
        arg_matches.and_then(|am| am.value_of("name")),
        "What would you like the backup to be called?",
    )?;
    Ok(())
}

fn load(op1: Op1, name: String) -> Result<()> {
    Ok(())
}
