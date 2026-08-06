use assert_cmd::Command;
use predicates::str::contains;

#[tokio::test]
async fn run_git_status_returns_branch() {
    // Ensure we are in a git repository (the project itself)
    Command::cargo_bin("openclaude")
        .unwrap()
        .args(&["run", "git", "status"])
        .assert()
        .success()
        .stdout(contains("On branch"));
}
