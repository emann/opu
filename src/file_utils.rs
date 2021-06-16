use crate::op1::OP1Image;
use fs_extra::dir::{copy_with_progress, create_all, get_size, CopyOptions, TransitProcessResult};
use fs_extra::error::Result;
use fs_extra::{copy_items_with_progress, TransitProcess};
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

fn progress_callback(pb: ProgressBar) -> impl Fn(TransitProcess) -> TransitProcessResult {
    move |tp: TransitProcess| -> TransitProcessResult {
        if pb.length() == 0 {
            pb.set_length(tp.total_bytes);
        }
        if tp.copied_bytes < tp.total_bytes {
            // TODO: Maybe remove
            pb.set_position(tp.copied_bytes);
        } else {
            pb.finish();
        }
        TransitProcessResult::ContinueOrAbort
    }
}

pub(crate) fn copy_dir_with_progress_bar<P, Q>(src: &[P], dest: Q) -> Result<u64>
where
    P: AsRef<Path> + Debug,
    Q: AsRef<Path> + Debug,
{
    create_all(&dest, true)?;
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:.cyan/blue}] {bytes}/{total_bytes}"),
    );

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    copy_items_with_progress(src, dest, &options, progress_callback(pb))
}
