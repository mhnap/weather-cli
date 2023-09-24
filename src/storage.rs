use std::env;

use log::debug;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const DEFAULT_CONFIG_NAME: &str = "config";

// Read config name from env in debug mode, needed for testing.
#[cfg(debug_assertions)]
fn config_name() -> String {
    if let Ok(config_name) = env::var("CONFIG_NAME") {
        config_name
    } else {
        DEFAULT_CONFIG_NAME.into()
    }
}

// Do not read config name from env in release mode.
#[cfg(not(debug_assertions))]
fn config_name() -> String {
    DEFAULT_CONFIG_NAME.into()
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct Provider {
    name: String,
    api_key: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct Config {
    providers: Vec<Provider>,
}

#[derive(Debug)]
pub struct Storage {
    config: Config,
}

impl Storage {
    pub fn load() -> Self {
        let config = confy::load(APP_NAME, config_name().as_str()).expect("cannot load config");
        Storage { config }
    }

    pub fn is_provider_configured(&self, name: &str) -> bool {
        self.config
            .providers
            .iter()
            .any(|provider| provider.name == name)
    }

    pub fn configure_provider(mut self, name: String, api_key: String) {
        if let Some(provider) = self
            .config
            .providers
            .iter_mut()
            .find(|provider| provider.name == name)
        {
            debug!("reconfigured \"{name}\" provider");
            provider.api_key = api_key;
        } else {
            debug!("configured \"{name}\" provider");
            self.config.providers.push(Provider { name, api_key });
        }

        confy::store(APP_NAME, config_name().as_str(), self.config).expect("cannot store config");
    }
}
