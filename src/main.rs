#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

use std::{env, thread, time};

use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgGroup};
use color_eyre::eyre::{ContextCompat, Result};
use console::style;
use dialoguer::console::Term;
use indicatif::{ProgressBar, ProgressStyle};

use functions::FUNCTIONS;
use prompt::{get_input, select};

use crate::op1::OP1Image;

mod backup;
mod file_utils;
mod functions;
mod load;
mod op1;
mod prompt;

// TODO: Config.toml, backup dir

fn main() -> Result<()> {
    color_eyre::install().expect("Could not set up error handling with color_eyre");

    let matches = App::new("opu")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            App::new("backup")
                .about("Saves a copy of the files on the OP-1 to be restored at a later date.")
                .arg(Arg::with_name("name").help("What to call the backup")),
        )
        .subcommand(
            App::new("load")
                .about("Loads a previously saved backup onto the OP-1")
                .arg(Arg::with_name("name").help(
                    "The name of the backup stored in OPU's configured storage path to be loaded",
                ))
                .arg(Arg::with_name("path").help("The path to the backup to be loaded"))
                .group(ArgGroup::with_name("name_or_path_to_backup").args(&["name", "path"])),
        )
        .get_matches();

    let connected_op1 = match OP1Image::find_connected_op1() {
        Some(op1) => op1,
        None => {
            let pb = ProgressBar::new(1000);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("No OP-1 detected. Waiting for one to be connected {spinner:.green}"),
            );
            pb.enable_steady_tick(10);

            while let None = OP1Image::find_connected_op1() {
                thread::sleep(time::Duration::from_millis(250));
            }
            pb.finish();
            OP1Image::find_connected_op1().unwrap()
        }
    };

    // TODO: Clean this up, probably by using the ColorfulTheme directly
    let term = Term::stdout();
    term.write_line(&format!(
        "{}",
        style(format!(
            "{} Found OP-1 @{:?}",
            style("✔".to_string()).for_stderr().green(),
            connected_op1.root_dir
        ))
        .for_stderr()
        .bold()
        .bright()
    ))?;

    let functions = FUNCTIONS.to_vec();

    let function = match matches.subcommand_name() {
        None => select(functions, "Select a function"),
        Some(name) => functions
            .into_iter()
            .find(|w| w.name == name)
            .wrap_err_with(|| format!("Unknown function: {}", name)),
    }
    .expect("Unable to determine function");

    // TODO: Collect required args that are missing
    (function.function)(connected_op1)
}
