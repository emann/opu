use std::fmt::Debug;
use std::path::Path;

use fs_extra::dir::{copy_with_progress, CopyOptions, TransitProcess, TransitProcessResult};
use fs_extra::error::Result as FsExtraResult;

pub fn copy_dir_contents_with_progress<P, Q, F>(
    from: P,
    to: Q,
    progress_handler: F,
) -> FsExtraResult<u64>
where
    P: AsRef<Path> + Debug,
    Q: AsRef<Path> + Debug,
    F: FnMut(TransitProcess) -> TransitProcessResult,
{
    println!("Copying contents of {:?} to {:?}", from, to);

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    copy_with_progress(from, to, &options, progress_handler)
}

pub fn copy_items_with_progress<P, Q, F>(
    from: &[P],
    to: Q,
    progress_handler: F,
) -> FsExtraResult<u64>
where
    P: AsRef<Path> + Debug,
    Q: AsRef<Path> + Debug,
    F: FnMut(fs_extra::TransitProcess) -> TransitProcessResult,
{
    println!("Copying {:?} to {:?}", from, to);

    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    fs_extra::copy_items_with_progress(from, to, &options, progress_handler)
}
