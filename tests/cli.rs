use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

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
fn configure_help_flag() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["configure", "-h"])
        .assert()
        .success()
        .stdout(contains(
            "possible values: open-weather, weather-api, accu-weather",
        ));

    Ok(())
}

#[test]
fn configure_wrong_provider() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["configure", "unknown"])
        .assert()
        .failure()
        .stderr(contains("invalid value 'unknown' for '<PROVIDER>'"));

    Ok(())
}

#[cfg(not(target_os = "windows"))]
mod not_windows_tests {
    // Currently, tests use env vars to get real API keys and make requests during tests.
    // This is possibly a bad decision as keys can expire or invalidate on CI.
    // Or the API provider can throttle or block us after many tests.
    // Another approach would be to mock providers' APIs with fake ones in tests.
    // But for now, leave it as it is to save time.

    use super::*;
    use rand::distributions::{Alphanumeric, DistString};
    use std::collections::HashMap;
    use std::env;

    const TIMEOUT_MS: Option<u64> = Some(1000);

    fn rand_string(len: usize) -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), len)
    }

    fn providers_with_keys() -> Result<HashMap<&'static str, String>> {
        Ok([
            ("open-weather", env::var("OPEN_WEATHER_KEY")?),
            ("weather-api", env::var("WEATHER_API_KEY")?),
            ("accu-weather", env::var("ACCU_WEATHER_KEY")?),
        ]
        .into())
    }

    #[test]
    fn configure_provider() -> Result<()> {
        let config_name = rand_string(8);
        let providers_with_keys = providers_with_keys()?;
        let first_provider = providers_with_keys.iter().next().unwrap();
        let second_provider = providers_with_keys.iter().nth(1).unwrap();

        // Configure provider first time.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.env("CONFIG_NAME", &config_name);
        cmd.args(["configure", first_provider.0]);

        let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Input provider API key:")?;
        p.send_line(first_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        // Try to reconfigure provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.env("CONFIG_NAME", &config_name);
        cmd.args(["configure", first_provider.0]);

        let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Do you want to reconfigure?")?;
        p.send_line("n")?;
        p.exp_string("Provider configuration has not changed.")?;
        p.exp_eof()?;

        // Reconfigure provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.env("CONFIG_NAME", &config_name);
        cmd.args(["configure", first_provider.0]);

        let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Do you want to reconfigure?")?;
        p.send_line("y")?;
        p.exp_string("Input provider API key:")?;
        p.send_line(first_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        // Configure another provider.

        let mut cmd = Command::cargo_bin(BIN_NAME)?;
        cmd.env("CONFIG_NAME", &config_name);
        cmd.args(["configure", second_provider.0]);

        let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
        p.exp_string("Input provider API key:")?;
        p.send_line(second_provider.1)?;
        p.exp_string("Successfully saved provider configuration.")?;
        p.exp_eof()?;

        Ok(())
    }

    #[test]
    fn correct_configure_provider() -> Result<()> {
        let config_name = rand_string(8);

        for (provider, key) in providers_with_keys()? {
            let mut cmd = Command::cargo_bin(BIN_NAME)?;
            cmd.env("CONFIG_NAME", &config_name);
            cmd.args(["configure", provider]);

            let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
            p.exp_string("Input provider API key:")?;
            p.send_line(&key)?;
            p.exp_string("Successfully saved provider configuration.")?;
            p.exp_eof()?;
        }

        Ok(())
    }

    #[test]
    fn incorrect_configure_provider() -> Result<()> {
        let config_name = rand_string(8);

        for (provider, _) in providers_with_keys()? {
            let mut cmd = Command::cargo_bin(BIN_NAME)?;
            cmd.env("CONFIG_NAME", &config_name);
            cmd.args(["configure", provider]);

            let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
            p.exp_string("Input provider API key:")?;
            p.send_line("wrong key")?;
            p.exp_string("Incorrect provider API key.")?;
            p.exp_eof()?;
        }

        Ok(())
    }
}
