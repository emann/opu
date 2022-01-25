use std::env;
use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

use crate::dirs::get_projects_dir;
use crate::file_utils::copy_items_with_progress_bar;
use crate::op1::OP1;
use crate::project::Project;
use crate::prompt::{prompt_select, unwrap_or_prompt_input};
use clap::ArgMatches;

// TODO: Get list of OP1 images in backup dir and pass to a select prompt
// TODO: Warn about overwrite, offer to save first
pub(crate) fn collect_args_and_run(arg_matches: Option<&ArgMatches>, op1: OP1) -> Result<()> {
    // let backup_name = unwrap_or_prompt_input(
    //     arg_matches.and_then(|am| am.value_of("name")),
    //     "What would you like the backup to be called?",
    // )?;

    let project = prompt_select(
        Project::get_all_projects_in_dir(get_projects_dir()),
        "Select a project to load",
    )
    .unwrap();
    op1.load(project);
    Ok(())
}
