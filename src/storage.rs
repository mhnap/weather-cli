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

    pub fn configure_provider<N, K>(mut self, name: N, api_key: K)
    where
        N: Into<String>,
        K: Into<String>,
    {
        let name = name.into();
        let api_key = api_key.into();

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

#[cfg(test)]
mod tests {
    use rand::distributions::{Alphanumeric, DistString};

    use super::*;

    fn rand_string(len: usize) -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), len)
    }

    #[test]
    fn configure_provider() {
        env::set_var("CONFIG_NAME", rand_string(8));
        let storage = Storage::load();

        assert!(storage.config.providers.is_empty());
        assert!(!storage.is_provider_configured("provider"));

        // Configure provider first time.

        storage.configure_provider("provider", "api_key");
        let storage = Storage::load();
        assert_eq!(storage.config.providers.len(), 1);
        assert!(storage.is_provider_configured("provider"));
        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.name, "provider");
        assert_eq!(provider.api_key, "api_key");

        // Reconfigure provider.

        storage.configure_provider("provider", "new_api_key");
        let storage = Storage::load();
        assert_eq!(storage.config.providers.len(), 1);
        assert!(storage.is_provider_configured("provider"));
        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.name, "provider");
        assert_eq!(provider.api_key, "new_api_key");

        // Configure another provider.

        storage.configure_provider("another_provider", "another_api_key");
        let storage = Storage::load();
        assert_eq!(storage.config.providers.len(), 2);
        assert!(storage.is_provider_configured("another_provider"));
        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.name, "another_provider");
        assert_eq!(provider.api_key, "another_api_key");
    }
}
