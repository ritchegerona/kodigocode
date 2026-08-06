use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn cli_shows_help() {
    Command::cargo_bin("kodigocode")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("OpenClaude CLI"));
}