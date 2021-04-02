use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn zero_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("code-generator")?;

    cmd.assert().success();

    Ok(())
}

#[test]
fn based_on_seed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("code-generator")?;

    cmd.arg("00");
    cmd.assert()
        .stdout(predicate::str::starts_with("00"))
        .success();

    Ok(())
}

#[test]
fn invalid_seed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("code-generator")?;

    cmd.arg("foo");
    cmd.assert().stdout("Could not generate the code\n").code(1);

    Ok(())
}
