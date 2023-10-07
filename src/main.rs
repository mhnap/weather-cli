#![deny(unused_must_use)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use dialoguer::{Confirm, Password, Select};
use proc_exit::Code;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::Unit;

use weather_cli::api;
use weather_cli::cli::{prelude::*, Cli, Command};
use weather_cli::data::{Provider, Weather};
use weather_cli::storage::Storage;

fn main() -> Result<()> {
    human_panic::setup_panic!();
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        Command::Configure { provider } => {
            configure_provider(provider)?;
        }
        Command::Get { provider, location } => {
            let provider = choose_active_provider(provider)?;
            let weather = get_weather(provider, &location)?;
            show_weather(weather);
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
            let Some(provider) = storage.get_active_provider() else {
                eprintln!("There is none configured provider.");
                Code::FAILURE.process_exit()
            };
            Ok(provider)
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

fn get_weather(provider: Provider, location: &str) -> Result<Weather> {
    let storage = Storage::load()?;
    let api_key = storage
        .get_api_key(provider)
        .expect("active provider should be configured");

    let api_provider: Box<dyn api::Provider> = provider.into();
    let locations = api_provider.search_location(api_key, location)?;
    let location = match locations.len() {
        0 => {
            eprintln!("Sorry, cannot find any location for the given input.");
            Code::FAILURE.process_exit()
        }
        1 => &locations[0],
        _ => {
            let selection = Select::new()
                .default(0)
                .items(&locations)
                .with_prompt("Several locations have been found, select one of them")
                .interact()?;
            &locations[selection]
        }
    };
    let weather = api_provider.get_weather(api_key, location)?;

    Ok(weather)
}

fn show_weather(weather: Weather) {
    println!(
        "{}, {:.0}{}",
        weather.description,
        weather.temperature.get::<degree_celsius>(),
        degree_celsius::abbreviation()
    );
}
