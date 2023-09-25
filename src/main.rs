use anyhow::{bail, Result};
use dialoguer::{Confirm, Password};
use human_panic::setup_panic;

use weather_cli::api::{AccuWeather, OpenWeather, Provider as ApiProvider, WeatherApi};
use weather_cli::cli::{prelude::*, Cli, Command, Provider};
use weather_cli::storage::Storage;

fn main() -> Result<()> {
    setup_panic!();
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        Command::Configure { provider } => {
            match provider {
                Provider::OpenWeather => configure_provider(OpenWeather),
                Provider::WeatherApi => configure_provider(WeatherApi),
                Provider::AccuWeather => configure_provider(AccuWeather),
            }?;
        }
        Command::Get => {
            todo!();
        }
    }

    Ok(())
}

fn configure_provider(provider: impl ApiProvider) -> Result<()> {
    let name = provider.name();
    let storage = Storage::load();

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

    let is_correct_api_key = provider.check_api_key(&api_key);
    if !is_correct_api_key {
        bail!("Incorrect provider API key.");
    }

    storage.configure_provider(name, api_key);
    println!("Successfully saved provider configuration.");

    Ok(())
}
