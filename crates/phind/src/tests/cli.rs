use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_cli_no_args() {
    let mut cmd = Command::cargo_bin("phind").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("./src"))
        .stdout(predicate::str::contains("./target"));
}

#[test]
fn test_cli_nonexistent_directory() {
    let mut cmd = Command::cargo_bin("phind").unwrap();
    cmd.arg("nonexistent")
        .assert()
        .failure();
}

#[test]
fn test_cli_with_directory() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    fs::create_dir(temp_path.join("test_dir")).unwrap();
    fs::write(temp_path.join("test_file.txt"), "").unwrap();

    let mut cmd = Command::cargo_bin("phind").unwrap();
    cmd.arg(temp_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("test_dir"))
        .stdout(predicate::str::contains("test_file.txt"));
}
