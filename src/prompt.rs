use std::fmt::Display;

use clap::ArgMatches;
use color_eyre::eyre::{eyre, ContextCompat, Result, WrapErr};
use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

pub(crate) fn unwrap_and_validate_or_prompt_select<T: Display>(
    value: Option<&str>,
    mut items: Vec<T>,
    prompt: &str,
) -> Result<T> {
    match value {
        Some(val) => items
            .into_iter()
            .find(|v| v.to_string() == val)
            .ok_or(eyre!("Invalid value: {}", val)),
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&items)
                .default(0)
                .with_prompt(prompt)
                .interact_on_opt(&Term::stdout())?;

            match selection {
                Some(index) => {
                    let item = items.remove(index);
                    Ok(item)
                }
                None => Err(eyre!("No option selected")),
            }
        }
    }
}

pub(crate) fn unwrap_or_prompt_input(value: Option<&str>, prompt: &str) -> Result<String> {
    match value {
        Some(val) => Ok(val.to_string()),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .wrap_err("Failed to get input from user"),
    }
}
