use super::Provider;

pub struct AccuWeather;

impl Provider for AccuWeather {
    const NAME: &'static str = "AccuWeather";

    fn check_api_key(&self, api_key: &str) -> bool {
        api_key != "key"
    }
}
