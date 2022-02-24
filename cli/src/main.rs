#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

use clap::{App, Arg, ArgGroup};
use color_eyre::eyre::Result;
use console::style;
use dialoguer::console::Term;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Config;
use commands::COMMANDS;
use opu_core::op1::OP1;
use opu_core::OPUConfig;
use prompt::unwrap_and_validate_or_prompt_select;

mod commands;
mod config;
mod load;
mod prompt;
mod save;
mod utils;

// TODO: Config.toml, backup dir
// TODO: Daemon to automatically open when op-1 detected

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().expect("Could not set up error handling with color_eyre");

    let config: Config = Config::load()?;

    let matches = App::new("opu")
        .subcommand(
            App::new("save")
                .about("Saves a copy of the files on the OP-1 to be restored at a later date.")
                .arg(Arg::new("name").help("What to call the project")),
        )
        .subcommand(
            App::new("load")
                .about("Loads a previously saved project onto the OP-1")
                .arg(Arg::new("name").help(
                    "The name of the project stored in OPU's configured storage path to be loaded",
                ))
                .arg(Arg::new("path").help("The path to the project to be loaded"))
                .group(ArgGroup::new("name_or_path_to_project").args(&["name", "path"])),
        )
        .get_matches();

    // Get the connected OP1
    let pb = ProgressBar::new(1000);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("No OP-1 detected. Waiting for one to be connected {spinner:.green}"),
    );
    pb.enable_steady_tick(10);
    let connected_op1 = OP1::get_connected_op1().await;
    pb.finish();

    // TODO: Clean this up, probably by using the ColorfulTheme directly
    let term = Term::stdout();
    term.write_line(&format!(
        "{}",
        style(format!(
            "{} Found OP-1 @{:?}",
            style("✔".to_string()).for_stderr().green(),
            connected_op1.mount_point()
        ))
        .for_stderr()
        .bold()
        .bright()
    ))?;

    let command = unwrap_and_validate_or_prompt_select(
        matches.subcommand_name(),
        COMMANDS.to_vec(),
        "Select a command",
    )
    .expect("Unable to determine command");

    (command.collect_args_and_run)(
        config,
        connected_op1,
        matches.subcommand_matches(command.name),
    )
}
