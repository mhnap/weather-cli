use super::Provider;

pub struct WeatherApi;

impl Provider for WeatherApi {
    const NAME: &'static str = "WeatherApi";

    fn check_api_key(&self, api_key: &str) -> bool {
        api_key != "key"
    }
}
