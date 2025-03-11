use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::env;

// Helper function to create a test config file
fn setup_test_config() {
    let home_dir = env::var("HOME").unwrap();
    let config_dir = PathBuf::from(home_dir).join(".config").join("oura-cli");
    
    // Create config directory if it doesn't exist
    fs::create_dir_all(&config_dir).unwrap();
    
    // Create a test config file with a placeholder token
    let config_content = r#"oura_token = "placeholder_for_testing""#;
    fs::write(config_dir.join("config.toml"), config_content).unwrap();
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("oura-cli").unwrap();
    cmd.arg("--version");
    cmd.assert().success().stdout(predicate::str::contains("oura-cli"));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("oura-cli").unwrap();
    cmd.arg("--help");
    cmd.assert().success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("configure"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("latest"))
        .stdout(predicate::str::contains("score"));
}

// Mock tests for main.rs functionality
#[test]
fn test_cli_no_args() {
    // Setup test config first
    setup_test_config();
    
    let mut cmd = Command::cargo_bin("oura-cli").unwrap();
    cmd.assert().success();
}

#[test]
fn test_cli_configure_command() {
    let mut cmd = Command::cargo_bin("oura-cli").unwrap();
    cmd.arg("configure")
        .arg("--oura-token")
        .arg("placeholder_for_testing");
    
    cmd.assert().success();
}

#[test]
fn test_cli_show_command() {
    // Setup test config first
    setup_test_config();
    
    let mut cmd = Command::cargo_bin("oura-cli").unwrap();
    cmd.arg("show");
    cmd.assert().success()
        .stdout(predicate::str::contains("Oura token:"));
}

#[test]
fn test_cli_score_command_with_dates() {
    // Setup test config first
    setup_test_config();
    
    let mut cmd = Command::cargo_bin("oura-cli").unwrap();
    cmd.arg("score")
        .arg("--start-date")
        .arg("2023-01-01")
        .arg("--end-date")
        .arg("2023-01-02");
    
    // We expect this to succeed with an error message about parsing
    // We're just testing that the CLI parses the arguments correctly
    cmd.assert().success();
}
