use std::path::Path;

use fs_extra::dir::{
    copy_with_progress, create_all, CopyOptions, TransitProcess, TransitProcessResult,
};
use fs_extra::error::Result;

pub fn copy_dir_with_progress<P, Q, F>(from: P, to: Q, progress_handler: F) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(TransitProcess) -> TransitProcessResult,
{
    create_all(&to, true)?;

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    copy_with_progress(from, to, &options, progress_handler)
}
