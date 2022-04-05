use assert_cmd::Command;

#[test]
fn test() -> anyhow::Result<()> {
    Command::cargo_bin("c")?
        .assert()
        .failure()
        .stderr(predicates::str::contains("SUBCOMMANDS:"))
        .stderr(predicates::str::contains("issue"));
    Command::cargo_bin("c")?
        .arg("issue")
        .assert()
        .failure()
        .stderr(predicates::str::contains("SUBCOMMANDS:"))
        .stderr(predicates::str::contains("add"))
        .stderr(predicates::str::contains("list"));
    Command::cargo_bin("c")?
        .args(&["issue", "add"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Add!"));
    Command::cargo_bin("c")?
        .args(&["issue", "list"])
        .assert()
        .success()
        .stdout(predicates::str::contains("List!"));
    Ok(())
}
