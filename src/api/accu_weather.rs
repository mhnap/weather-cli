use super::{construct_url, Provider, ProviderImpl};
use reqwest::{
    blocking::{get, Response},
    Result,
};
use std::collections::HashMap;

pub struct AccuWeather;

impl Provider for AccuWeather {
    const NAME: &'static str = "AccuWeather";
}

impl ProviderImpl for AccuWeather {
    fn test_call(api_key: &str, q: &str) -> Result<()> {
        locations_cities_search(api_key, q)?;
        Ok(())
    }
}

const HOST: &str = "https://dataservice.accuweather.com";

fn locations_cities_search(api_key: &str, q: &str) -> Result<Response> {
    let url = construct_url(
        HOST,
        vec!["locations", "v1", "cities", "search"],
        HashMap::from([("apikey", api_key), ("q", q)]),
    );

    get(url)?.error_for_status()
}
