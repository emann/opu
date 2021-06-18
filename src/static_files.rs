use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/static"]
pub(crate) struct StaticFiles;
