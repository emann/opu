#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]

use iced::pure::Element;
use iced::{Command, Length, Sandbox};
use include_flate::flate;
use opu_core::config::OPUConfig;

use crate::components::wait_for_op1::wait_for_op1;
use crate::components::Page;
use crate::config::Config;
use iced::pure::{container, Application};
use opu_core::op1::OP1;
use opu_core::project::Project;
use std::path::PathBuf;

mod components;
mod config;
mod loading;
mod style;

flate!(static OP1_FONT_BYTES: [u8] from "assets/op1_font.ttf");

fn main() -> iced::Result {
    // TODO: Handle errors when trying to load config
    let config = Config::load()
        .expect("Should be able to load config (will be handled better in the future");

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

#[derive(Debug)]
enum Message {
    OP1Found(OP1),
}

struct OPU {
    config: Config,
    page: Page<Message>,
}

impl Application for OPU {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Config;

    fn new(config: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                config: config.clone(),
                page: Page::WaitForOP1(wait_for_op1(config, Message::OP1Found)),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("OPU - {}", self.page)
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::OP1Found(op1) => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(wait_for_op1(self.config.clone(), Message::OP1Found))
            .padding(20)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
