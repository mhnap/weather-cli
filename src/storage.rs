use std::path::Path;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::data::{Location, Provider};
use crate::error::Result;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const DEFAULT_CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug)]
struct ProviderData {
    kind: Provider,
    api_key: String,
    saved_location: Option<Location>,
}

// NOTE: Order of fields does matter.
#[derive(Deserialize, Serialize, Default, Debug)]
struct Config {
    active_provider: Option<Provider>,
    providers: Vec<ProviderData>,
}

#[derive(Debug)]
pub struct Storage {
    config: Config,
    changed: bool,
}

impl Storage {
    pub fn load(path: Option<impl AsRef<Path>>) -> Result<Self> {
        let config = match path {
            None => confy::load(APP_NAME, DEFAULT_CONFIG_NAME),
            Some(path) => confy::load_path(path),
        }?;
        Ok(Self {
            config,
            changed: false,
        })
    }

    pub fn store(self, path: Option<impl AsRef<Path>>) -> Result<()> {
        // Store config only if changed.
        if self.changed {
            match path {
                None => confy::store(APP_NAME, DEFAULT_CONFIG_NAME, self.config),
                Some(path) => confy::store_path(path, self.config),
            }?;
        }
        Ok(())
    }

    pub fn is_provider_configured(&self, kind: Provider) -> bool {
        self.config.providers.iter().any(|p| p.kind == kind)
    }

    pub fn configure_provider(&mut self, kind: Provider, api_key: String) {
        if let Some(provider) = self.config.providers.iter_mut().find(|p| p.kind == kind) {
            provider.api_key = api_key;
            debug!("reconfigured \"{kind:?}\" provider");
        } else {
            self.config.providers.push(ProviderData {
                kind,
                api_key,
                saved_location: None,
            });
            debug!("configured \"{kind:?}\" provider");
        }
        self.mark_provider_active(kind);
        self.changed = true;
    }

    pub fn mark_provider_active(&mut self, kind: Provider) {
        if self.config.active_provider != Some(kind) {
            self.config.active_provider = Some(kind);
            debug!("marked \"{kind:?}\" provider active");
            self.changed = true;
        }
    }

    pub fn get_active_provider(&self) -> Option<Provider> {
        self.config.active_provider
    }

    pub fn get_api_key(&self, kind: Provider) -> &str {
        self.config
            .providers
            .iter()
            .find(|p| p.kind == kind)
            .map(|p| p.api_key.as_str())
            .expect("provider should be configured")
    }

    pub fn save_location(&mut self, kind: Provider, location: Location) {
        self.config
            .providers
            .iter_mut()
            .find(|p| p.kind == kind)
            .expect("provider should be configured")
            .saved_location = Some(location);
        debug!("saved location for \"{kind:?}\" provider");
        self.changed = true;
    }

    pub fn get_saved_location(&self, kind: Provider) -> Option<&Location> {
        self.config
            .providers
            .iter()
            .find(|p| p.kind == kind)?
            .saved_location
            .as_ref()
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::NamedTempFile;

    use crate::data::Provider::{OpenWeather, WeatherApi};

    use super::*;

    #[test]
    fn configure_provider() {
        let config = NamedTempFile::new("config").unwrap();
        let path = Some(config.path());
        let mut storage = Storage::load(path).unwrap();

        assert!(storage.config.active_provider.is_none());
        assert!(storage.config.providers.is_empty());
        assert!(!storage.is_provider_configured(OpenWeather));

        // Configure provider first time.

        storage.configure_provider(OpenWeather, "api_key".into());
        assert_eq!(storage.config.providers.len(), 1);
        assert!(storage.is_provider_configured(OpenWeather));
        assert_eq!(storage.config.active_provider, Some(OpenWeather));
        assert_eq!(storage.get_active_provider(), Some(OpenWeather));

        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.kind, OpenWeather);
        assert_eq!(provider.api_key, "api_key");
        assert_eq!(storage.get_api_key(OpenWeather), "api_key");

        // Save location for first provider.

        storage.save_location(
            OpenWeather,
            Location {
                id: None,
                name: "first_location".to_string(),
                state: None,
                country: String::new(),
                lat: None,
                lon: None,
            },
        );
        assert_eq!(
            storage.get_saved_location(OpenWeather).unwrap().name,
            "first_location"
        );

        // Reconfigure provider.

        storage.configure_provider(OpenWeather, "new_api_key".into());
        assert_eq!(storage.config.providers.len(), 1);
        assert!(storage.is_provider_configured(OpenWeather));
        assert_eq!(storage.config.active_provider, Some(OpenWeather));
        assert_eq!(storage.get_active_provider(), Some(OpenWeather));

        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.kind, OpenWeather);
        assert_eq!(provider.api_key, "new_api_key");
        assert_eq!(storage.get_api_key(OpenWeather), "new_api_key");

        // Configure another provider.

        storage.configure_provider(WeatherApi, "another_api_key".into());
        assert_eq!(storage.config.providers.len(), 2);
        assert!(storage.is_provider_configured(WeatherApi));
        assert_eq!(storage.config.active_provider, Some(WeatherApi));
        assert_eq!(storage.get_active_provider(), Some(WeatherApi));

        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.kind, WeatherApi);
        assert_eq!(provider.api_key, "another_api_key");
        assert_eq!(storage.get_api_key(WeatherApi), "another_api_key");

        // Save location for second provider.

        storage.save_location(
            WeatherApi,
            Location {
                id: None,
                name: "second_location".to_string(),
                state: None,
                country: String::new(),
                lat: None,
                lon: None,
            },
        );
        assert_eq!(
            storage.get_saved_location(WeatherApi).unwrap().name,
            "second_location"
        );

        // Mark provider as active.

        storage.mark_provider_active(OpenWeather);
        assert_eq!(storage.get_active_provider(), Some(OpenWeather));
        assert_eq!(storage.config.active_provider, Some(OpenWeather));

        // Store and reload.

        storage.store(path).unwrap();
        let storage = Storage::load(path).unwrap();

        let provider = storage.config.providers.first().unwrap();
        assert_eq!(provider.kind, OpenWeather);
        assert_eq!(provider.api_key, "new_api_key");
        assert_eq!(storage.get_api_key(OpenWeather), "new_api_key");

        let provider = storage.config.providers.last().unwrap();
        assert_eq!(provider.kind, WeatherApi);
        assert_eq!(provider.api_key, "another_api_key");
        assert_eq!(storage.get_api_key(WeatherApi), "another_api_key");

        assert_eq!(storage.get_active_provider(), Some(OpenWeather));
        assert_eq!(storage.config.active_provider, Some(OpenWeather));

        assert_eq!(
            storage.get_saved_location(OpenWeather).unwrap().name,
            "first_location"
        );
        assert_eq!(
            storage.get_saved_location(WeatherApi).unwrap().name,
            "second_location"
        );
    }
}
