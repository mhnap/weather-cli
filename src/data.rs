use crate::api;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uom::si::f64::ThermodynamicTemperature;

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

pub struct Weather {
    pub temperature: ThermodynamicTemperature,
    pub description: String,
}

pub struct Location {
    pub id: Option<String>,
    pub name: String,
    pub state: Option<String>,
    pub country: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(state) = self.state.as_ref() {
            write!(f, "{}, {}, {}", self.name, state, self.country)
        } else {
            write!(f, "{}, {}", self.name, self.country)
        }
    }
}
