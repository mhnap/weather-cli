use std::collections::HashMap;

use reqwest::blocking::{get, Response};
use serde::Deserialize;
use uom::si::f64::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::data::{self, Provider};
use crate::error::Result;

use super::{construct_url, Api};

pub struct WeatherApi {
    api_key: String,
}

impl WeatherApi {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl Api for WeatherApi {
    fn test_call(&self, q: &str) -> reqwest::Result<()> {
        search(&self.api_key, q)?;
        Ok(())
    }

    fn search_location(&self, location: &str) -> Result<Vec<data::Location>> {
        let response = search(&self.api_key, location)?;
        let locations: Vec<Location> = response.json()?;
        Ok(locations.into_iter().map(Into::into).collect())
    }

    fn get_weather(&self, location: &data::Location) -> Result<data::Weather> {
        let response = current(
            &self.api_key,
            location.lat.expect("lat should be set"),
            location.lon.expect("lon should be set"),
        )?;
        let weather: Weather = response.json()?;
        Ok(weather.into())
    }

    fn provider(&self) -> Provider {
        Provider::WeatherApi
    }
}

const HOST: &str = "https://api.weatherapi.com";

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
}

impl From<Location> for data::Location {
    fn from(value: Location) -> Self {
        Self {
            id: None,
            name: value.name,
            state: Some(value.region),
            country: value.country,
            lat: Some(value.lat),
            lon: Some(value.lon),
        }
    }
}

fn search(api_key: &str, q: &str) -> reqwest::Result<Response> {
    let url = construct_url(
        HOST,
        vec!["v1", "search.json"],
        HashMap::from([("key", api_key), ("q", q)]),
    );

    get(url)?.error_for_status()
}

#[derive(Deserialize, Debug)]
struct Weather {
    current: Current,
}

#[derive(Deserialize, Debug)]
struct Current {
    temp_c: f64,
    condition: Condition,
}

#[derive(Deserialize, Debug)]
struct Condition {
    text: String,
}

impl From<Weather> for data::Weather {
    fn from(value: Weather) -> Self {
        Self {
            temperature: ThermodynamicTemperature::new::<degree_celsius>(value.current.temp_c),
            description: value.current.condition.text,
        }
    }
}

fn current(api_key: &str, lat: f64, lon: f64) -> reqwest::Result<Response> {
    let url = construct_url(
        HOST,
        vec!["v1", "current.json"],
        HashMap::from([("key", api_key), ("q", &format!("{lat},{lon}"))]),
    );

    get(url)?.error_for_status()
}
