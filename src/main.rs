use human_panic::setup_panic;

use weather_cli::cli::{prelude::*, Cli};

fn main() {
    setup_panic!();

    env_logger::init();

    Cli::parse();
}
