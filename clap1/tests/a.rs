use assert_cmd::Command;

#[test]
fn test() -> anyhow::Result<()> {
    Command::cargo_bin("a")?
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "error: The following required arguments were not provided:",
        ))
        .stderr(predicates::str::contains("--name <NAME>"));
    Command::cargo_bin("a")?
        .args(&["--name", "my_name"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Hello my_name!"));
    Ok(())
}
