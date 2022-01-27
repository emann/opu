use fs_extra::dir::{TransitProcess, TransitProcessResult};
use indicatif::ProgressBar;

pub(crate) fn progress_callback(
    pb: ProgressBar,
) -> impl FnMut(TransitProcess) -> TransitProcessResult {
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
        TransitProcessResult::OverwriteAll
    }
}
