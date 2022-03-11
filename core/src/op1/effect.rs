use fruid::FromCLIInput;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, FromCLIInput)]
pub enum Effect {
    Cwo,
    Delay,
    Grid,
    Nitro,
    Phone,
    Punch,
    Spring,
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Effect::Cwo => "CWO",
            Effect::Delay => "DELAY",
            Effect::Grid => "GRID",
            Effect::Nitro => "NITRO",
            Effect::Phone => "PHONE",
            Effect::Punch => "PUNCH",
            Effect::Spring => "SPRING",
        })
    }
}

impl Default for Effect {
    fn default() -> Self {
        Effect::Cwo
    }
}
