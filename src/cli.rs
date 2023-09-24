use clap::{Parser, Subcommand, ValueEnum, Args};

pub mod prelude {
    pub use clap::Parser;
}

/// Simple weather CLI.
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Configure credentials for the provider.
    Configure {
        #[arg(value_enum)]
        provider: Provider
    },
    /// Show weather for the provided address.
    Get,
}

#[derive(ValueEnum, Clone)]
enum Provider {
    OpenWeather,
    WeatherApi,
    AccuWeather,
}
