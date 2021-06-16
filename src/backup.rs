use color_eyre::Result;

use crate::file_utils::copy_dir_with_progress_bar;
use crate::op1::OP1Image;
use crate::prompt::get_input;
use color_eyre::eyre::WrapErr;
use std::env;
use std::path::{Path, PathBuf};

pub(crate) fn backup(op1: OP1Image) -> Result<()> {
    let s = get_input("What do you want the backup to be called?")?;

    let dest = env::current_dir()?.join(s);
    println!("Saving to {:?}", dest);

    for subdir in op1.subdirs().iter() {
        copy_dir_with_progress_bar(subdir, dest.join(subdir.file_name().unwrap()))?;
    }
    Ok(())
}
