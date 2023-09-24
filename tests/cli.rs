use anyhow::Result;
use assert_cmd::Command;
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
fn configure_command_wrong_provider() -> Result<()> {
    Command::cargo_bin(BIN_NAME)?
        .args(["configure", "unknown"])
        .assert()
        .failure()
        .stderr(contains("invalid value 'unknown' for '<PROVIDER>'"));
    Ok(())
}
