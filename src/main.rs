#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use anyhow::Result;
use dialoguer::{Confirm, Password};
use weather_cli::api::{AccuWeather, OpenWeather, Provider as ApiProvider, WeatherApi};
use weather_cli::cli::{prelude::*, Cli, Command, Provider};
use weather_cli::storage::Storage;

fn main() -> Result<()> {
    human_panic::setup_panic!();
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        Command::Configure { provider } => {
            // TODO: Think something better?
            match provider {
                Provider::OpenWeather => configure_provider::<OpenWeather>(),
                Provider::WeatherApi => configure_provider::<WeatherApi>(),
                Provider::AccuWeather => configure_provider::<AccuWeather>(),
            }?;
        }
        Command::Get => {
            todo!();
        }
    }

    Ok(())
}

fn configure_provider<P: ApiProvider>() -> Result<()> {
    let name = P::NAME;
    let storage = Storage::load()?;

    if storage.is_provider_configured(name) {
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

    let is_correct_api_key = P::validate_api_key(&api_key)?;
    if !is_correct_api_key {
        println!("Incorrect provider API key.");
        return Ok(());
    }

    storage.configure_provider(name, api_key)?;
    println!("Successfully saved provider configuration.");

    Ok(())
}
