#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

use std::fs::{read, File};
use std::path::Path;
use std::{env, thread, time};

use clap::{App, Arg, ArgGroup};
use color_eyre::eyre::{ContextCompat, Result};
use console::style;
use dialoguer::console::Term;
use indicatif::{ProgressBar, ProgressStyle};

use commands::COMMANDS;
use core::metadata::Metadata;
use core::op1::OP1;
use core::static_files::StaticFiles;
use prompt::{unwrap_and_validate_or_prompt_select, unwrap_or_prompt_input};

mod commands;
// mod load;
mod prompt;
mod save;

// TODO: Config.toml, backup dir
// TODO: Daemon to automatically open when op-1 detected

fn main() -> Result<()> {
    color_eyre::install().expect("Could not set up error handling with color_eyre");

    let matches = App::new("opu")
        .subcommand(
            App::new("save")
                .about("Saves a copy of the files on the OP-1 to be restored at a later date.")
                .arg(Arg::new("name").help("What to call the project")),
        )
        .subcommand(
            App::new("load")
                .about("Loads a previously saved backup onto the OP-1")
                .arg(Arg::new("name").help(
                    "The name of the backup stored in OPU's configured storage path to be loaded",
                ))
                .arg(Arg::new("path").help("The path to the backup to be loaded"))
                .group(ArgGroup::new("name_or_path_to_backup").args(&["name", "path"])),
        )
        .get_matches();

    let connected_op1 = match OP1::find_connected_op1() {
        Some(op1) => op1,
        None => {
            let pb = ProgressBar::new(1000);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("No OP-1 detected. Waiting for one to be connected {spinner:.green}"),
            );
            pb.enable_steady_tick(10);

            let mut op1 = OP1::find_connected_op1();
            while let None = op1 {
                op1 = OP1::find_connected_op1();
                thread::sleep(time::Duration::from_millis(100));
            }
            pb.finish();
            OP1::find_connected_op1().unwrap()
        }
    };

    // TODO: Clean this up, probably by using the ColorfulTheme directly
    let term = Term::stdout();
    term.write_line(&format!(
        "{}",
        style(format!(
            "{} Found OP-1 @{:?}",
            style("✔".to_string()).for_stderr().green(),
            connected_op1.op1_dirs.parent_dir
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

    (command.collect_args_and_run)(matches.subcommand_matches(command.name), connected_op1)
}