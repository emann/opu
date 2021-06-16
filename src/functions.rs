use color_eyre::eyre::Result;

use crate::backup::backup;
use crate::load::load;
use crate::op1::OP1Image;

pub(crate) static FUNCTIONS: &[Function] = &[
    Function {
        name: "backup",
        function: backup,
    },
    Function {
        name: "load",
        function: load,
    },
];

type OPUFunction = fn(op1: OP1Image) -> Result<()>;

#[derive(Clone)]
pub(crate) struct Function<'a> {
    /// The display name of this Function. Shown in the selector or passed as an argument to the cli
    pub(crate) name: &'a str,
    /// The rust function to be called with the supplied parameters
    pub(crate) function: OPUFunction,
}

impl std::fmt::Display for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
