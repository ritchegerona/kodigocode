use assert_cmd::Command;
use predicates::str::contains;

#[tokio::test]
async fn run_git_status_returns_branch() {
    Command::cargo_bin("kc")
        .unwrap()
        .args(["run", "git", "status"])
        .assert()
        .success()
        .stdout(contains("On branch"));
}