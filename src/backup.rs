use std::env;
use std::path::{Path, PathBuf};

use color_eyre::eyre::WrapErr;
use color_eyre::Result;

use crate::file_utils::copy_dir_with_progress_bar;
use crate::op1::OP1Image;
use crate::prompt::get_input;

pub(crate) fn backup(op1: OP1Image) -> Result<()> {
    let backup_name = get_input("What do you want the backup to be called?")?;

    let dest = env::current_dir()?.join(backup_name);
    copy_dir_with_progress_bar(&op1.subdirs(), &dest)?;
    println!("Backup saved to{:?}", dest);
    Ok(())
}
