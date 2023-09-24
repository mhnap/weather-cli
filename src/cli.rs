use clap::{Parser, Subcommand, ValueEnum};
use derive_more::Display;

pub mod prelude {
    pub use clap::Parser;
}

/// Simple weather CLI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Configure credentials for the provider.
    Configure {
        #[arg(value_enum)]
        provider: Provider,
    },
    /// Show weather for the provided address.
    Get,
}

#[derive(ValueEnum, Clone, Display, Debug)]
pub enum Provider {
    OpenWeather,
    WeatherApi,
    AccuWeather,
}
