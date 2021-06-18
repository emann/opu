use color_eyre::eyre::Result;

use crate::create;
use crate::load;
use crate::op1::Op1;
use clap::ArgMatches;

pub(crate) static COMMANDS: &[Command] = &[
    Command {
        name: "create",
        collect_args_and_run: create::collect_args_and_run,
    },
    Command {
        name: "load",
        collect_args_and_run: load::collect_args_and_run,
    },
];

type OPUCommand = fn(arg_matches: Option<&ArgMatches>, op1: Op1) -> Result<()>;

#[derive(Clone)]
pub(crate) struct Command<'a> {
    /// The display name of this Function. Shown in the selector or passed as an argument to the cli
    pub(crate) name: &'a str,
    /// The rust function to be called with the supplied parameters
    pub(crate) collect_args_and_run: OPUCommand,
}

impl std::fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
