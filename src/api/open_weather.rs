use std::collections::HashMap;

use reqwest::blocking::{get, Response};
use serde::Deserialize;
use uom::si::f64::ThermodynamicTemperature;
use uom::si::thermodynamic_temperature::kelvin;

use crate::data;
use crate::error::{Error, Result};

use super::{construct_url, Provider};

pub struct OpenWeather;

impl Provider for OpenWeather {
    fn test_call(&self, api_key: &str, q: &str) -> reqwest::Result<()> {
        geo_direct(api_key, q, true)?;
        Ok(())
    }

    fn search_location(&self, api_key: &str, location: &str) -> Result<Vec<data::Location>> {
        let response = geo_direct(api_key, location, false)?;
        let locations: Vec<Location> = response.json()?;
        Ok(locations.into_iter().map(Into::into).collect())
    }

    fn get_weather(&self, api_key: &str, location: &data::Location) -> Result<data::Weather> {
        let response = data_weather(
            api_key,
            location.lat.expect("missing lat"),
            location.lon.expect("missing lon"),
        )?;
        let weather: Weather = response.json()?;
        weather.try_into()
    }
}

const HOST: &str = "https://api.openweathermap.org";

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    lat: f64,
    lon: f64,
    country: String,
    state: Option<String>,
}

impl From<Location> for data::Location {
    fn from(value: Location) -> Self {
        Self {
            id: None,
            name: value.name,
            state: value.state,
            country: value.country,
            lat: Some(value.lat),
            lon: Some(value.lon),
        }
    }
}

fn geo_direct(api_key: &str, q: &str, limit: bool) -> reqwest::Result<Response> {
    let url = construct_url(
        HOST,
        vec!["geo", "1.0", "direct"],
        HashMap::from([
            ("appid", api_key),
            ("q", q),
            ("limit", if limit { "1" } else { "0" }),
        ]),
    );

    get(url)?.error_for_status()
}

#[derive(Deserialize, Debug)]
struct Weather {
    weather: Vec<WeatherData>,
    main: Temperature,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    main: String,
}

#[derive(Deserialize, Debug)]
struct Temperature {
    temp: f64,
}

impl TryFrom<Weather> for data::Weather {
    type Error = Error;

    fn try_from(mut value: Weather) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            temperature: ThermodynamicTemperature::new::<kelvin>(value.main.temp),
            description: value.weather.pop().ok_or(Error::BadResponse)?.main,
        })
    }
}

fn data_weather(api_key: &str, lat: f64, lon: f64) -> reqwest::Result<Response> {
    let url = construct_url(
        HOST,
        vec!["data", "2.5", "weather"],
        HashMap::from([
            ("appid", api_key),
            ("lat", &lat.to_string()),
            ("lon", &lon.to_string()),
        ]),
    );

    get(url)?.error_for_status()
}
