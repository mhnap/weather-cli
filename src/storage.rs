use log::debug;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const CONFIG_NAME: &str = "config";

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
        let config = confy::load(APP_NAME, CONFIG_NAME).expect("cannot load config");
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

        confy::store(APP_NAME, CONFIG_NAME, self.config).expect("cannot store config");
    }
}
