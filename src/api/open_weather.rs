use super::Provider;

pub struct OpenWeather;

impl Provider for OpenWeather {
    const NAME: &'static str = "OpenWeather";

    fn check_api_key(&self, api_key: &str) -> bool {
        api_key != "key"
    }
}
