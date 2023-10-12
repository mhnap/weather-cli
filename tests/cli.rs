use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use assert_fs::NamedTempFile;
use predicates::str::contains;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[test]
fn help_flag() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("Simple weather CLI"))
        .stdout(contains("configure"))
        .stdout(contains("get"))
        .stdout(contains("help"))
        .stdout(contains("-h, --help"))
        .stdout(contains("-V, --version"));

    Ok(())
}

#[test]
fn version_flag() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .arg("--version")
        .assert()
        .success()
        .stdout(contains(format!("{BIN_NAME} {VERSION}")));

    Ok(())
}

#[test]
fn configure_command_help_flag() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["configure", "-h"])
        .assert()
        .success()
        .stdout(contains("Configure credentials for the provider"))
        .stdout(contains("<PROVIDER>  Specific weather API provider [possible values: open-weather, weather-api, accu-weather]"))
        .stdout(contains("-c, --config <CONFIG>  Path to config file"));

    Ok(())
}

#[test]
fn configure_command_wrong_provider() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["configure", "unknown"])
        .assert()
        .failure()
        .stderr(contains("invalid value 'unknown' for '<PROVIDER>'"));

    Ok(())
}

#[test]
fn configure_command_no_provider() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .arg("configure")
        .assert()
        .failure()
        .stderr(contains(
            "the following required arguments were not provided:\n  <PROVIDER>",
        ));

    Ok(())
}

#[test]
fn get_command_help_flag() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["get", "-h"])
        .assert()
        .success()
        .stdout(contains("Show weather by location"))
        .stdout(contains("[LOCATION]  Choose a location (city, town, or village) and save the choice per provider"))
        .stdout(contains("-p, --provider <PROVIDER>  Choose an active provider and save the choice [possible values: open-weather, weather-api, accu-weather]"))
        .stdout(contains("-c, --config <CONFIG>      Path to config file"));

    Ok(())
}

#[test]
fn get_command_wrong_provider() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["get", "-punknown"])
        .assert()
        .failure()
        .stderr(contains(
            "invalid value 'unknown' for '--provider <PROVIDER>'",
        ));

    Ok(())
}

#[test]
fn get_command_without_configured_provider() -> Result<()> {
    let config = NamedTempFile::new("config").unwrap();

    Command::cargo_bin(BIN_NAME)?
        .arg("get")
        .args(["-c", config.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(contains("None of the providers is configured."));

    Command::cargo_bin(BIN_NAME)?
        .args(["get", "Kyiv"])
        .args(["-c", config.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(contains("None of the providers is configured."));

    Command::cargo_bin(BIN_NAME)?
        .args(["get", "Kyiv", "-popen-weather"])
        .args(["-c", config.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(contains("Provider is not configured."));

    Ok(())
}

#[cfg(not(target_os = "windows"))]
mod not_windows_tests {
    // Currently, tests use env vars to get real API keys and make requests during tests.
    // This is possibly a bad decision as keys can expire or invalidate on CI.
    // Or the API provider can throttle or block us after many tests.
    // Another approach would be to mock providers' APIs with fake ones in tests.
    // But for now, leave it as it is to save time.

    use std::collections::HashMap;
    use std::env;

    use rexpect::session::spawn_command;

    use super::*;

    const TIMEOUT_MS: Option<u64> = Some(10000);

    fn providers_with_keys() -> Result<HashMap<&'static str, String>> {
        Ok([
            ("open-weather", env::var("OPEN_WEATHER_KEY")?),
            ("weather-api", env::var("WEATHER_API_KEY")?),
            // It's intended that "accu-weather" is third because it has the smallest API limits.
            ("accu-weather", env::var("ACCU_WEATHER_KEY")?),
        ]
        .into())
    }

    #[test]
    fn reconfigure_provider() -> Result<()> {
        let config = NamedTempFile::new("config").unwrap();
        let providers_with_keys = providers_with_keys()?;
        let first_provider = providers_with_keys.iter().next().unwrap();
        let second_provider = providers_with_keys.iter().nth(1).unwrap();

        // Configure provider first time.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["configure", first_provider.0])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Input provider API key")?;
        p.send_line(first_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        // Try to reconfigure provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["configure", first_provider.0])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Provider is already configured.")?;
        p.exp_string("Do you want to reconfigure?")?;
        p.send_line("n")?;
        p.exp_string("Provider configuration has not changed.")?;
        p.exp_eof()?;

        // Reconfigure provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["configure", first_provider.0])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Provider is already configured.")?;
        p.exp_string("Do you want to reconfigure?")?;
        p.send_line("y")?;
        p.exp_string("Input provider API key")?;
        p.send_line(first_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        // Configure another provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["configure", second_provider.0])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Input provider API key")?;
        p.send_line(second_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        Ok(())
    }

    #[test]
    fn configure_provider_correctly_and_get_weather() -> Result<()> {
        let config = NamedTempFile::new("config").unwrap();

        for (provider, key) in providers_with_keys()? {
            let mut cmd = Command::cargo_bin(BIN_NAME)?;
            cmd.args(["configure", provider])
                .args(["-c", config.to_str().unwrap()]);

            let mut p = spawn_command(cmd, TIMEOUT_MS)?;
            p.exp_string("Input provider API key")?;
            p.send_line(&key)?;
            p.exp_string("Successfully saved provider configuration.")?;
            p.exp_eof()?;

            // Try to get weather for nonexistent location.

            let mut cmd = Command::cargo_bin(BIN_NAME)?;
            cmd.args(["get", "nonexistent"])
                .args(["-c", config.to_str().unwrap()]);

            let mut p = spawn_command(cmd, TIMEOUT_MS)?;
            p.exp_string("Sorry, cannot find any location for the given input.")?;
            p.exp_eof()?;

            // Get weather for one location.

            let mut cmd = Command::cargo_bin(BIN_NAME)?;
            cmd.args(["get", "Ternopil"])
                .args(["-c", config.to_str().unwrap()]);

            let mut p = spawn_command(cmd, TIMEOUT_MS)?;
            p.exp_string("Ternopil")?;
            p.exp_string("°C")?;
            p.exp_eof()?;

            // Choose a location and get the weather for it.

            let mut cmd = Command::cargo_bin(BIN_NAME)?;
            cmd.args(["get", "London"])
                .args(["-c", config.to_str().unwrap()]);

            let mut p = spawn_command(cmd, TIMEOUT_MS)?;
            p.exp_string("Several locations have been found, select one of them")?;
            p.send_line(" ")?;
            p.exp_string("London")?;
            p.exp_string("°C")?;
            p.exp_eof()?;
        }

        Ok(())
    }

    #[test]
    fn configure_provider_incorrectly() -> Result<()> {
        let config = NamedTempFile::new("config").unwrap();

        for (provider, _) in providers_with_keys()? {
            let mut cmd = Command::cargo_bin(BIN_NAME)?;
            cmd.args(["configure", provider])
                .args(["-c", config.to_str().unwrap()]);

            let mut p = spawn_command(cmd, TIMEOUT_MS)?;
            p.exp_string("Input provider API key")?;
            p.send_line("wrong key")?;
            p.exp_string("Incorrect provider API key.")?;
            p.exp_eof()?;
        }

        Ok(())
    }

    #[test]
    fn save_location() -> Result<()> {
        let config = NamedTempFile::new("config").unwrap();
        let providers_with_keys = providers_with_keys()?;
        let first_provider = providers_with_keys.iter().next().unwrap();
        let second_provider = providers_with_keys.iter().nth(1).unwrap();

        // Configure provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["configure", first_provider.0])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Input provider API key")?;
        p.send_line(first_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        // Try to get weather for saved location.

        Command::cargo_bin(BIN_NAME)?
            .arg("get")
            .args(["-c", config.to_str().unwrap()])
            .assert()
            .failure()
            .stderr(contains("No saved location for active provider."));

        // Get weather and save location.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["get", "Ternopil"])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Ternopil")?;
        p.exp_string("°C")?;
        p.exp_eof()?;

        // Get weather for saved location.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.arg("get").args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Ternopil")?;
        p.exp_string("°C")?;
        p.exp_eof()?;

        //
        // Another provider can save its own location.
        //

        // Configure provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["configure", second_provider.0])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Input provider API key")?;
        p.send_line(second_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        // Try to get weather for saved location.

        Command::cargo_bin(BIN_NAME)?
            .arg("get")
            .args(["-c", config.to_str().unwrap()])
            .assert()
            .failure()
            .stderr(contains("No saved location for active provider."));

        // Get weather and save location.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["get", "London"])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Several locations have been found, select one of them")?;
        p.send_line(" ")?;
        p.exp_string("London")?;
        p.exp_string("°C")?;
        p.exp_eof()?;

        // Get weather for saved location.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.arg("get").args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("London")?;
        p.exp_string("°C")?;
        p.exp_eof()?;

        // Saved location of the first provider has not changed.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.args(["get", "-p", first_provider.0])
            .args(["-c", config.to_str().unwrap()]);

        let mut p = spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Ternopil")?;
        p.exp_string("°C")?;
        p.exp_eof()?;

        Ok(())
    }
}
