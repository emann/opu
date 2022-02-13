#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]

use iced::{Application, Command};
use include_flate::flate;
use triax_ui::Stage as StageImpl;

mod loading;
mod stages;
mod style;

flate!(static OP1_FONT_BYTES: [u8] from "assets/op1_font.ttf");

#[derive(Default, Debug, Clone)]
pub struct Config {
    placeholder: u8,
}

fn main() -> iced::Result {
    let config = Config::from_file().unwrap();
    let mut settings = iced::Settings::with_flags(config);
    settings.default_font = Some(&OP1_FONT_BYTES);
    settings.antialiasing = true;
    OPU::run(settings)
}

impl Config {
    pub fn from_file() -> Result<Self, String> {
        // Should be added to main
        // TODO: Add to main once a replacement for impl_main is written
        // TODO: Actually generate default and/or read from file
        Ok(Self::default())
    }
}

#[allow(dead_code)]
fn get_mode(_: &Config) -> iced::window::Mode {
    iced::window::Mode::Fullscreen
}

#[allow(dead_code)]
fn get_bg(_: &Config) -> iced::Color {
    style::colors::hardware::LIGHT_GRAY
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
