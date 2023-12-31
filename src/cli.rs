use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::data::Provider;

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
        /// Specific weather API provider.
        #[arg(value_enum)]
        provider: Provider,

        /// Path to config file.
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    /// Show weather by location.
    Get {
        /// Choose an active provider and save the choice.
        #[arg(short, long)]
        provider: Option<Provider>,

        /// Choose a location (city, town, or village) and save the choice per provider.
        location: Option<String>,

        /// Path to config file.
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}
