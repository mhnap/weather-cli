use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::str::contains;
use rand::distributions::{Alphanumeric, DistString};

const BIN_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const TIMEOUT_MS: Option<u64> = Some(1000);

fn rand_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

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
fn configure_wrong_provider() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["configure", "unknown"])
        .assert()
        .failure()
        .stderr(contains("invalid value 'unknown' for '<PROVIDER>'"));

    Ok(())
}

#[cfg(not(target_os = "windows"))]
#[test]
fn configure_provider() -> Result<()> {
    let config_name = rand_string(8);

    // Configure provider first time.

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("CONFIG_NAME", &config_name);
    cmd.args(["configure", "open-weather"]);

    let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
    p.exp_string("Input provider API key:")?;
    p.send_line("some valid key")?;
    p.exp_string("Successfully saved provider configuration.")?;
    p.exp_eof()?;

    // Try to reconfigure provider.

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("CONFIG_NAME", &config_name);
    cmd.args(["configure", "open-weather"]);

    let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
    p.exp_string("Do you want to reconfigure?")?;
    p.send_line("n")?;
    p.exp_string("Provider configuration has not changed.")?;
    p.exp_eof()?;

    // Reconfigure provider.

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("CONFIG_NAME", &config_name);
    cmd.args(["configure", "open-weather"]);

    let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
    p.exp_string("Do you want to reconfigure?")?;
    p.send_line("y")?;
    p.exp_string("Input provider API key:")?;
    p.send_line("some another valid key")?;
    p.exp_string("Successfully saved provider configuration.")?;
    p.exp_eof()?;

    // Configure another provider.

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("CONFIG_NAME", &config_name);
    cmd.args(["configure", "weather-api"]);

    let mut p = rexpect::session::spawn_command(cmd, TIMEOUT_MS)?;
    p.exp_string("Input provider API key:")?;
    p.send_line("some valid key")?;
    p.exp_string("Successfully saved provider configuration.")?;
    p.exp_eof()?;

    Ok(())
}
