use assert_cmd::Command;

#[test]
fn test() -> anyhow::Result<()> {
    Command::cargo_bin("b")?
        .assert()
        .failure()
        .stderr(predicates::str::contains("SUBCOMMANDS:"))
        .stderr(predicates::str::contains("add"))
        .stderr(predicates::str::contains("remove"));
    Command::cargo_bin("b")?
        .arg("add")
        .assert()
        .success()
        .stdout(predicates::str::contains("Add!"));
    Command::cargo_bin("b")?
        .arg("remove")
        .assert()
        .success()
        .stdout(predicates::str::contains("Remove!!"));
    Ok(())
}
