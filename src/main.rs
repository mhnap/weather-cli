use anyhow::Result;
use dialoguer::{Confirm, Password};
use human_panic::setup_panic;

use weather_cli::cli::{prelude::*, Cli, Command, Provider};
use weather_cli::storage::Storage;

fn main() -> Result<()> {
    setup_panic!();
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        Command::Configure { provider } => {
            configure_provider(&provider)?;
        }
        Command::Get => {
            todo!();
        }
    }

    Ok(())
}

fn configure_provider(provider: &Provider) -> Result<()> {
    let name = provider.to_string();
    let storage = Storage::load();

    if storage.is_provider_configured(&name) {
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

    // TODO: Make one sample request to check whether provided API key is valid.

    storage.configure_provider(name, api_key);
    println!("Successfully saved provider configuration.");

    Ok(())
}
