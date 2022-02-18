mod dark;
mod light;

use crate::config::ThemeConfigValue;
use dark_light::Mode;
use iced::{container, radio, rule, scrollable, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    /// The light theme
    Light,
    /// The dark theme
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];

    pub fn background_color(&self) -> Color {
        match self {
            Theme::Light => light::BACKGROUND,
            Theme::Dark => dark::BACKGROUND,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            Theme::Light => light::ACTIVE,
            Theme::Dark => dark::ACTIVE,
        }
    }
}

impl std::default::Default for Theme {
    fn default() -> Theme {
        match dark_light::detect() {
            Mode::Light => Theme::Light,
            Mode::Dark => Theme::Dark,
        }
    }
}

impl From<ThemeConfigValue> for Theme {
    fn from(config_value: ThemeConfigValue) -> Self {
        match config_value {
            ThemeConfigValue::Auto => Theme::default(),
            ThemeConfigValue::Light => Theme::Light,
            ThemeConfigValue::Dark => Theme::Dark,
        }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Container.into(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Radio.into(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Scrollable.into(),
            Theme::Dark => dark::Scrollable.into(),
        }
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Rule.into(),
            Theme::Dark => dark::Rule.into(),
        }
    }
}
