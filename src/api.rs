use crate::error::Result;
use std::collections::HashMap;
use url::Url;

mod accu_weather;
mod open_weather;
mod weather_api;

pub use accu_weather::AccuWeather;
pub use open_weather::OpenWeather;
pub use weather_api::WeatherApi;

const DEFAULT_CITY: &str = "Kyiv";

pub trait Provider: ProviderImpl {
    const NAME: &'static str;

    fn validate_api_key(api_key: &str) -> Result<bool> {
        let result = Self::test_call(api_key, DEFAULT_CITY);
        if let Some(status_code) = result.as_ref().err().and_then(|e| e.status()) {
            if status_code == 401 || status_code == 403 {
                return Ok(false);
            }
        }
        result?;

        Ok(true)
    }
}

pub trait ProviderImpl {
    fn test_call(api_key: &str, q: &str) -> reqwest::Result<()>;
}

fn construct_url(host: &str, path_segments: Vec<&str>, query_pairs: HashMap<&str, &str>) -> Url {
    let mut url = Url::parse(host).expect("Cannot parse host");

    for path_segment in path_segments {
        url.path_segments_mut()
            .expect("Cannot mutate path segments")
            .push(path_segment);
    }

    for (key, value) in query_pairs {
        url.query_pairs_mut().append_pair(key, value);
    }

    url
}
