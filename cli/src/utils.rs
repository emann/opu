let pb = ProgressBar::new(0);
pb.set_style(
ProgressStyle::default_bar()
.template("{spinner:.green} [{bar:.cyan/blue}] {bytes}/{total_bytes}"),
);

pub fn progress_callback(pb: ProgressBar) -> impl FnMut(TransitProcess) -> TransitProcessResult {
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
