use super::{construct_url, Provider, ProviderImpl};
use reqwest::{
    blocking::{get, Response},
    Result,
};
use std::collections::HashMap;

pub struct WeatherApi;

impl Provider for WeatherApi {
    const NAME: &'static str = "WeatherApi";
}

impl ProviderImpl for WeatherApi {
    fn test_call(api_key: &str, q: &str) -> Result<()> {
        current(api_key, q)?;
        Ok(())
    }
}

const HOST: &str = "https://api.weatherapi.com";

fn current(api_key: &str, q: &str) -> Result<Response> {
    let url = construct_url(
        HOST,
        vec!["v1", "current.json"],
        HashMap::from([("key", api_key), ("q", q)]),
    );

    get(url)?.error_for_status()
}
