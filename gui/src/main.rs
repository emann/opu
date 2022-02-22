#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]

use iced::{Application, Command};
use include_flate::flate;
use opu_core::config::OPUConfig;
use triax_ui::Stage as StageImpl;

use crate::config::Config;
use opu_core::metadata::{Metadata, MixerSettings, TempoSettings};
use opu_core::project::Project;
use std::path::PathBuf;

mod config;
mod loading;
mod stages;
mod style;

flate!(static OP1_FONT_BYTES: [u8] from "assets/op1_font.ttf");

fn test(config: &Config) {
    let projects = Project::get_all_projects_in_dir(config.project_library());
    for p in projects {
        println!("Found project: {}", p);
        use std::time::Instant;
        let now = Instant::now();
        let hashes = p.op1_dirs.get_hashes();
        println!("Elapsed: {:.2?}", now.elapsed());
        let hashes2 = hashes.clone();
        println!("Hashes ({}): {:?}", hashes.len(), hashes);
        let changed_files: Vec<PathBuf> = hashes
            .into_iter()
            .filter_map(|(relative_path, hash)| match hashes2.get(&relative_path) {
                Some(hash) => None,
                _ => Some(relative_path),
            })
            .collect();
        println!("{:?}", changed_files);
    }
}

fn main() -> iced::Result {
    // TODO: Handle errors when trying to load config
    let config = Config::load()
        .expect("Should be able to load config (will be handled better in the future");

    test(&config);

    let mut settings = iced::Settings::with_flags(config);
    settings.default_font = Some(&OP1_FONT_BYTES);
    settings.antialiasing = true;

    OPU::run(settings)
}

#[allow(dead_code)]
fn get_mode(_: &Config) -> iced::window::Mode {
    iced::window::Mode::Fullscreen
}

#[allow(dead_code)]
fn get_bg(config: &Config) -> iced::Color {
    config.theme().background_color()
}

#[allow(dead_code)]
fn get_scale(_: &Config) -> f64 {
    2.0
}

triax_ui::triax_application!(
    impl_main(false);
    name(OPU);
    title("OPU");
    background_color(get_bg)
    first_stage(stages::WaitForOP1ToBeConnected);
    stages(
        stages::WaitForOP1ToBeConnected -> stages::SelectOperation,
        stages::SelectOperation -> stages::Load,
        stages::Load -> stages::SelectOperation
    );
    flags(Config, Config::from_file);
);
