pub use accu_weather::AccuWeather;
pub use open_weather::OpenWeather;
pub use weather_api::WeatherApi;

mod accu_weather;
mod open_weather;
mod weather_api;

pub trait Provider {
    const NAME: &'static str;

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn check_api_key(&self, api_key: &str) -> bool;
}
