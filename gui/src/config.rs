use crate::style::Theme;
use opu_core::{CoreConfig, OPUConfig};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
struct GuiConfig {
    theme: ThemeConfigValue,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Config {
    core: CoreConfig,
    gui: GuiConfig,
}

impl Config {
    pub fn theme(&self) -> Theme {
        return self.gui.theme.into();
    }
}

impl AsRef<CoreConfig> for Config {
    fn as_ref(&self) -> &CoreConfig {
        &self.core
    }
}

impl OPUConfig for Config {}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum ThemeConfigValue {
    /// Use whatever the system theme is
    Auto,
    /// The light theme
    Light,
    /// The dark theme
    Dark,
}

impl std::default::Default for ThemeConfigValue {
    fn default() -> Self {
        ThemeConfigValue::Auto
    }
}
