use reqwest::blocking::{get, Response};
use serde::Deserialize;
use uom::si::f64::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::data::{self, Provider};
use crate::error::{Error, Result};

use super::{construct_url, has_status_code, Api};

pub struct AccuWeather {
    api_key: String,
}

impl AccuWeather {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl Api for AccuWeather {
    fn is_valid(&self) -> Result<bool> {
        has_status_code(locations_cities_search(&self.api_key, "Kyiv"), 401)
    }

    fn search_location(&self, q: &str) -> Result<Vec<data::Location>> {
        let response = locations_cities_search(&self.api_key, q)?;
        let locations: Vec<Location> = response.json()?;
        Ok(locations.into_iter().map(Into::into).collect())
    }

    fn get_weather(&self, location: &data::Location) -> Result<data::Weather> {
        let response = current_conditions(
            &self.api_key,
            location.id.as_ref().expect("id should be set"),
        )?;
        let mut weathers: Vec<Weather> = response.json()?;
        if let Some(weather) = weathers.pop() {
            Ok(weather.into())
        } else {
            Err(Error::BadResponse)
        }
    }

    fn provider(&self) -> Provider {
        Provider::AccuWeather
    }
}

const HOST: &str = "https://dataservice.accuweather.com";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Location {
    key: String,
    localized_name: String,
    country: Country,
    administrative_area: AdministrativeArea,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Country {
    localized_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct AdministrativeArea {
    localized_name: String,
}

impl From<Location> for data::Location {
    fn from(value: Location) -> Self {
        Self {
            id: Some(value.key),
            name: value.localized_name,
            state: Some(value.administrative_area.localized_name),
            country: value.country.localized_name,
            lat: None,
            lon: None,
        }
    }
}

fn locations_cities_search(api_key: &str, q: &str) -> reqwest::Result<Response> {
    let url = construct_url(
        HOST,
        &["locations", "v1", "cities", "search"],
        &[("apikey", api_key), ("q", q)],
    )
    .expect("static url should be valid");

    get(url)?.error_for_status()
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Weather {
    weather_text: String,
    temperature: Temperature,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Temperature {
    metric: Metric,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Metric {
    value: f64,
}

impl From<Weather> for data::Weather {
    fn from(value: Weather) -> Self {
        Self {
            temperature: ThermodynamicTemperature::new::<degree_celsius>(
                value.temperature.metric.value,
            ),
            description: value.weather_text,
        }
    }
}

fn current_conditions(api_key: &str, location_key: &str) -> reqwest::Result<Response> {
    let url = construct_url(
        HOST,
        &["currentconditions", "v1", location_key],
        &[("apikey", api_key)],
    )
    .expect("static url should be valid");

    get(url)?.error_for_status()
}
