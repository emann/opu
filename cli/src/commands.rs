use color_eyre::eyre::Result;

use crate::save;
use crate::{load, Config};
use clap::ArgMatches;
use opu_core::op1::OP1;

pub static COMMANDS: &[Command] = &[
    Command {
        name: "save",
        collect_args_and_run: save::collect_args_and_run,
    },
    Command {
        name: "load",
        collect_args_and_run: load::collect_args_and_run,
    },
];

type OPUCommand = fn(config: Config, op1: OP1, arg_matches: Option<&ArgMatches>) -> Result<()>;

#[derive(Clone)]
pub struct Command<'a> {
    /// The display name of this Function. Shown in the selector or passed as an argument to the cli
    pub name: &'a str,
    /// The rust function to be called with the supplied parameters
    pub collect_args_and_run: OPUCommand,
}

impl std::fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
