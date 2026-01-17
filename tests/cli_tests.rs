use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("ghr").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Modern GitHub CLI with TUI interface"));
}

#[test]
fn test_auth_help() {
    let mut cmd = Command::cargo_bin("ghr").unwrap();
    cmd.arg("auth").arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Authenticate with GitHub"));
}

#[test]
fn test_ls_help() {
    let mut cmd = Command::cargo_bin("ghr").unwrap();
    cmd.arg("ls").arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("List repositories"));
}

#[test]
fn test_artifacts_help() {
    let mut cmd = Command::cargo_bin("ghr").unwrap();
    cmd.arg("artifacts").arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage artifacts"));
}
