use std::collections::HashMap;

use url::Url;

pub use accu_weather::AccuWeather;
pub use open_weather::OpenWeather;
pub use weather_api::WeatherApi;

use crate::data::{Location, Provider, Weather};
use crate::error::Result;

mod accu_weather;
mod open_weather;
mod weather_api;

pub trait Api {
    fn is_valid(&self) -> Result<bool> {
        let result = self.test_call("Kyiv");
        if let Some(status_code) = result.as_ref().err().and_then(|e| e.status()) {
            if status_code == 401 || status_code == 403 {
                return Ok(false);
            }
        }
        result?;

        Ok(true)
    }

    fn test_call(&self, q: &str) -> reqwest::Result<()>;

    fn search_location(&self, q: &str) -> Result<Vec<Location>>;

    fn get_weather(&self, location: &Location) -> Result<Weather>;

    fn provider(&self) -> Provider;
}

fn construct_url(host: &str, path_segments: Vec<&str>, query_pairs: HashMap<&str, &str>) -> Url {
    let mut url = Url::parse(host).expect("static urls should be valid");

    for path_segment in path_segments {
        url.path_segments_mut()
            .expect("static urls should be valid")
            .push(path_segment);
    }

    for (key, value) in query_pairs {
        url.query_pairs_mut().append_pair(key, value);
    }

    url
}

pub fn new(provider: Provider, api_key: String) -> Box<dyn Api> {
    match provider {
        Provider::OpenWeather => Box::new(OpenWeather::new(api_key)),
        Provider::WeatherApi => Box::new(WeatherApi::new(api_key)),
        Provider::AccuWeather => Box::new(AccuWeather::new(api_key)),
    }
}
