use opu_core::CoreConfig;
pub use opu_core::OPUConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    core: CoreConfig,
}

impl AsRef<CoreConfig> for Config {
    fn as_ref(&self) -> &CoreConfig {
        &self.core
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            core: CoreConfig::default(),
        }
    }
}

impl OPUConfig for Config {}
