use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/static/"]
pub struct StaticFiles;
