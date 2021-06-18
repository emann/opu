use std::env;
use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

use crate::file_utils::copy_dir_with_progress_bar;
use crate::op1::OP1Image;
use crate::prompt::unwrap_or_prompt_input;
use clap::ArgMatches;

pub(crate) fn collect_args_and_run(arg_matches: Option<&ArgMatches>, op1: OP1Image) -> Result<()> {
    let backup_name = unwrap_or_prompt_input(
        arg_matches.and_then(|am| am.value_of("name")),
        "What would you like the backup to be called?",
    )?;

    save(op1, backup_name)
}

fn save(op1: OP1Image, name: String) -> Result<()> {
    let dest = env::current_dir()?.join(name);
    copy_dir_with_progress_bar(&op1.subdirs(), &dest)?;
    println!("Backup saved to{:?}", dest);
    Ok(())
}
