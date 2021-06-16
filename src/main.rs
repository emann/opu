#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

mod backup;
mod file_utils;
mod functions;
mod load;
mod op1;
mod prompt;

use crate::op1::OP1Image;
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use functions::FUNCTIONS;
use indicatif::{ProgressBar, ProgressStyle};
use prompt::{get_input, select};
use std::borrow::Borrow;
use std::path::PathBuf;
use std::{env, fs, thread, time};
use sysinfo::{DiskExt, ProcessExt, SystemExt};

// TODO: Config.toml, backup dir

fn main() -> Result<()> {
    color_eyre::install().expect("Could not set up error handling with color_eyre");

    let matches = App::new("opu")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("FUNCTION").help("the function").index(1))
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

    println!("Found OP-1 @{:?}", connected_op1.root_dir);

    let preselected_function = matches.value_of("FUNCTION");

    let functions = FUNCTIONS.to_vec();

    let function = match preselected_function {
        None => select(functions, "Select a function"),
        Some(name) => functions
            .into_iter()
            .find(|w| w.name == name)
            .wrap_err_with(|| format!("Unknown function: {}", name)),
    }
    .expect("Unable to determine function");

    // TODO: Collect additional args (proj name, select stuff?)
    (function.function)(connected_op1)
}
