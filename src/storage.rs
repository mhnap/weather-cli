use crate::data::Provider;
use crate::error::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::env;

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

#[derive(Deserialize, Serialize, Debug)]
struct ProviderData {
    kind: Provider,
    api_key: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct Config {
    providers: Vec<ProviderData>,
}

#[derive(Debug)]
pub struct Storage {
    config: Config,
}

impl Storage {
    pub fn load() -> Result<Self> {
        let config = confy::load(APP_NAME, config_name().as_str())?;

        Ok(Storage { config })
    }

    pub fn is_provider_configured(&self, kind: Provider) -> bool {
        self.config
            .providers
            .iter()
            .any(|provider| provider.kind == kind)
    }

    pub fn configure_provider(mut self, kind: Provider, api_key: String) -> Result<()> {
        if let Some(provider) = self
            .config
            .providers
            .iter_mut()
            .find(|provider| provider.kind == kind)
        {
            debug!("reconfigured \"{kind:?}\" provider");
            provider.api_key = api_key;
        } else {
            debug!("configured \"{kind:?}\" provider");
            self.config.providers.push(ProviderData { kind, api_key });
        }

        confy::store(APP_NAME, config_name().as_str(), self.config)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Provider::{OpenWeather, WeatherApi};
    use crate::error::Result;
    use rand::distributions::{Alphanumeric, DistString};

    fn rand_string(len: usize) -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), len)
    }

    #[test]
    fn configure_provider() -> Result<()> {
        env::set_var("CONFIG_NAME", rand_string(8));
        let storage = Storage::load()?;

        assert!(storage.config.providers.is_empty());
        assert!(!storage.is_provider_configured(OpenWeather));

        // Configure provider first time.

        storage.configure_provider(OpenWeather, "api_key".into())?;
        let storage = Storage::load()?;
        assert_eq!(storage.config.providers.len(), 1);
        assert!(storage.is_provider_configured(OpenWeather));
        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.kind, OpenWeather);
        assert_eq!(provider.api_key, "api_key");

        // Reconfigure provider.

        storage.configure_provider(OpenWeather, "new_api_key".into())?;
        let storage = Storage::load()?;
        assert_eq!(storage.config.providers.len(), 1);
        assert!(storage.is_provider_configured(OpenWeather));
        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.kind, OpenWeather);
        assert_eq!(provider.api_key, "new_api_key");

        // Configure another provider.

        storage.configure_provider(WeatherApi, "another_api_key".into())?;
        let storage = Storage::load()?;
        assert_eq!(storage.config.providers.len(), 2);
        assert!(storage.is_provider_configured(WeatherApi));
        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.kind, WeatherApi);
        assert_eq!(provider.api_key, "another_api_key");

        Ok(())
    }
}
