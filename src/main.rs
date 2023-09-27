#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use anyhow::Result;
use dialoguer::{Confirm, Password};
use weather_cli::api;
use weather_cli::cli::{prelude::*, Cli, Command};
use weather_cli::data::Provider;
use weather_cli::storage::Storage;

fn main() -> Result<()> {
    human_panic::setup_panic!();
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        Command::Configure { provider } => {
            configure_provider(provider)?;
        }
        Command::Get => {
            todo!();
        }
    }

    Ok(())
}

fn configure_provider(provider: Provider) -> Result<()> {
    let api_provider: Box<dyn api::Provider> = provider.into();
    let storage = Storage::load()?;

    if storage.is_provider_configured(provider) {
        println!("Provider is already configured.");
        let confirmation = Confirm::new()
            .with_prompt("Do you want to reconfigure?")
            .interact()?;
        if !confirmation {
            println!("Provider configuration has not changed.");
            return Ok(());
        }
    }

    let api_key: String = Password::new()
        .with_prompt("Input provider API key")
        .interact()?;

    let is_correct_api_key = api_provider.validate_api_key(&api_key)?;
    if !is_correct_api_key {
        eprintln!("Incorrect provider API key.");
        return Ok(());
    }

    storage.configure_provider(provider, api_key)?;
    println!("Successfully saved provider configuration.");

    Ok(())
}
