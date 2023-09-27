#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use anyhow::Result;
use dialoguer::{Confirm, Password};
use proc_exit::Code;
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
        Command::Get { provider } => {
            let provider = choose_active_provider(provider)?;
            dbg!(provider);
        }
    }

    Ok(())
}

fn configure_provider(provider: Provider) -> Result<()> {
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

    let api_provider: Box<dyn api::Provider> = provider.into();
    let is_correct_api_key = api_provider.validate_api_key(&api_key)?;
    if !is_correct_api_key {
        eprintln!("Incorrect provider API key.");
        Code::FAILURE.process_exit()
    }

    storage.configure_provider(provider, api_key)?;
    println!("Successfully saved provider configuration.");

    Ok(())
}

fn choose_active_provider(provider: Option<Provider>) -> Result<Provider> {
    let storage = Storage::load()?;
    match provider {
        None => {
            let Some(active_provider) = storage.get_active_provider() else {
                eprintln!("There is none configured provider.");
                Code::FAILURE.process_exit()
            };
            Ok(active_provider)
        }
        Some(provider) => {
            if storage.is_provider_configured(provider) {
                storage.mark_provider_active(provider)?;
                Ok(provider)
            } else {
                eprintln!("Provider is not configured.");
                Code::FAILURE.process_exit()
            }
        }
    }
}
