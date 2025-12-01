use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::process::Command; // Run programs // Used for writing assertions

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::new(env!("CARGO_BIN_EXE_catsay"))
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::new(env!("CARGO_BIN_EXE_catsay"))
        .args(["-f", "no/such/file.txt"])
        .assert()
        .failure();
    Ok(())
}
