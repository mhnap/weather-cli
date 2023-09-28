use super::{construct_url, Provider};
use crate::data;
use crate::error::{Error, Result};
use reqwest::blocking::{get, Response};
use serde::Deserialize;
use std::collections::HashMap;
use uom::si::f64::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::degree_celsius;

pub struct AccuWeather;

impl Provider for AccuWeather {
    fn test_call(&self, api_key: &str, q: &str) -> reqwest::Result<()> {
        locations_cities_search(api_key, q)?;
        Ok(())
    }

    fn search_location(&self, api_key: &str, q: &str) -> Result<Vec<data::Location>> {
        let response = locations_cities_search(api_key, q)?;
        let locations: Vec<Location> = response.json()?;
        Ok(locations.into_iter().map(Into::into).collect())
    }

    fn get_weather(&self, api_key: &str, location: &data::Location) -> Result<data::Weather> {
        let response = current_conditions(api_key, location.id.as_ref().expect("missing id"))?;
        let mut weathers: Vec<Weather> = response.json()?;
        if let Some(weather) = weathers.pop() {
            Ok(weather.into())
        } else {
            Err(Error::BadResponse)
        }
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
        vec!["locations", "v1", "cities", "search"],
        HashMap::from([("apikey", api_key), ("q", q)]),
    );

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
        vec!["currentconditions", "v1", location_key],
        HashMap::from([("apikey", api_key)]),
    );

    get(url)?.error_for_status()
}
