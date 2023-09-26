use super::{construct_url, Provider, ProviderImpl};
use reqwest::{
    blocking::{get, Response},
    Result,
};
use std::collections::HashMap;

pub struct OpenWeather;

impl Provider for OpenWeather {
    const NAME: &'static str = "OpenWeather";
}

impl ProviderImpl for OpenWeather {
    fn test_call(api_key: &str, q: &str) -> Result<()> {
        geo_direct(api_key, q)?;
        Ok(())
    }
}

const HOST: &str = "https://api.openweathermap.org";

fn geo_direct(api_key: &str, q: &str) -> Result<Response> {
    let url = construct_url(
        HOST,
        vec!["geo", "1.0", "direct"],
        HashMap::from([("appid", api_key), ("q", q)]),
    );

    get(url)?.error_for_status()
}
