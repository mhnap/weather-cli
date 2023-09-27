use crate::api;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, ValueEnum, Copy, Clone, Debug, PartialEq)]
pub enum Provider {
    OpenWeather,
    WeatherApi,
    AccuWeather,
}

impl From<Provider> for Box<dyn api::Provider> {
    fn from(value: Provider) -> Self {
        match value {
            Provider::OpenWeather => Box::new(api::OpenWeather),
            Provider::WeatherApi => Box::new(api::WeatherApi),
            Provider::AccuWeather => Box::new(api::AccuWeather),
        }
    }
}
