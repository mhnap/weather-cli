# weather-cli

Simple weather CLI

## Getting Started

### Install

Ensure you have `Rust` installed via [rustup](https://rustup.rs).

Use `cargo` to install `weather-cli` from the Git repository:

```
cargo install --git https://github.com/mhnap/weather-cli
```

Or you can clone the Git repository and build from the source:

```
git clone https://github.com/mhnap/weather-cli
cd weather-cli
cargo build --release
```

### Prepare

`weather-cli` relies on external API providers to query weather information.

Currently, `weather-cli` supports three API providers:

* [OpenWeather](https://openweathermap.org)
* [WeatherApi](https://www.weatherapi.com)
* [AccuWeather](https://developer.accuweather.com)

You should log into one API provider service and get an API key.

### Use

With a valid API key, you can now configure `weather-cli` to store this API key internally on your local system.

Example for configuring OpenWeather API provider:

```
weather-cli configure open-weather
```

`weather-cli` will interactively read your API key and save it for later use.

After configuring your API provider, you can get weather for any specific location.

Example for getting current weather in Kyiv city:

```
weather-cli get Kyiv
```

If the API provider finds multiple locations, you will be prompted to select one.
