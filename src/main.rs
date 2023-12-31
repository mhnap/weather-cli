#![deny(unused_must_use)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]

use anyhow::Result;
use dialoguer::{Confirm, Password, Select};
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::Unit;

use crate::cli::{prelude::*, Cli, Command};
use crate::data::{Location, Provider, Weather};
use crate::storage::Storage;
use crate::ui::{eprintln, get_style_for_weather, println, sprintln, theme, with_spinner};

mod api;
mod cli;
mod data;
mod error;
mod storage;
mod ui;

fn main() -> Result<()> {
    human_panic::setup_panic!();
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        Command::Configure { provider, config } => {
            let mut storage = Storage::load(config.as_ref())?;
            configure_provider(&mut storage, provider)?;
            storage.store(config)?;
        }
        Command::Get {
            provider,
            location,
            config,
        } => {
            let mut storage = Storage::load(config.as_ref())?;
            let provider = choose_active_provider(&mut storage, provider);

            let api_key = storage.get_api_key(provider).to_owned();
            let api = api::new(provider, api_key);

            let location = choose_location(&mut storage, api.as_ref(), location)?;
            show_location(location);

            let weather = with_spinner(|| api.get_weather(location))?;
            show_weather(&weather);
            storage.store(config)?;
        }
    }

    Ok(())
}

fn configure_provider(storage: &mut Storage, provider: Provider) -> Result<()> {
    if storage.is_provider_configured(provider) {
        println("Provider is already configured.");
        let confirmation = Confirm::with_theme(theme())
            .with_prompt("Do you want to reconfigure?")
            .interact()?;
        if !confirmation {
            println("Provider configuration has not changed.");
            return Ok(());
        }
    }

    let api_key: String = Password::with_theme(theme())
        .with_prompt("Input provider API key")
        .interact()?;
    let api = api::new(provider, api_key.clone());

    let is_correct_api_key = with_spinner(|| api.is_valid())?;
    if !is_correct_api_key {
        eprintln("Incorrect provider API key.")
    }

    storage.configure_provider(provider, api_key);
    sprintln("Successfully saved provider configuration.");

    Ok(())
}

fn choose_active_provider(storage: &mut Storage, provider: Option<Provider>) -> Provider {
    match provider {
        None => {
            let Some(provider) = storage.get_active_provider() else {
                eprintln("None of the providers is configured.")
            };
            provider
        }
        Some(provider) => {
            if storage.is_provider_configured(provider) {
                storage.mark_provider_active(provider);
                provider
            } else {
                eprintln("Provider is not configured.")
            }
        }
    }
}

fn choose_location<'a>(
    storage: &'a mut Storage,
    api: &dyn api::Api,
    location_str: Option<String>,
) -> Result<&'a Location> {
    let provider = api.provider();
    let location = match location_str {
        None => match storage.get_saved_location(provider) {
            None => eprintln("No saved location for active provider."),
            Some(location) => location,
        },
        Some(location_str) => {
            let mut locations = with_spinner(|| api.search_location(&location_str))?;
            let location = match locations.len() {
                0 => eprintln("Sorry, cannot find any location for the given input."),
                1 => locations.swap_remove(0),
                _ => {
                    let selection = Select::with_theme(theme())
                        .default(0)
                        .items(&locations)
                        .with_prompt("Several locations have been found, select one of them")
                        .report(false)
                        .interact()?;
                    locations.swap_remove(selection)
                }
            };
            storage.save_location(provider, location);
            storage
                .get_saved_location(provider)
                .expect("location should be saved")
        }
    };

    Ok(location)
}

fn show_location(location: &Location) {
    println(&format!(
        "Chosen location: {}",
        theme().defaults_style.apply_to(location)
    ));
}

fn show_weather(weather: &Weather) {
    let style = get_style_for_weather(&weather.description);
    let weather_str = format!(
        "{}, {:.0}{}",
        weather.description,
        weather.temperature.get::<degree_celsius>(),
        degree_celsius::abbreviation()
    );
    println(&format!("Current weather: {}", style.apply_to(weather_str)));
}
