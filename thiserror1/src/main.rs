use std::{error::Error, process::Termination};

#[derive(Debug, thiserror::Error)]
#[error("E1: {message}")]
struct E1 {
    message: String,
}

#[derive(Debug, thiserror::Error)]
#[error("E2")]
struct E2(
    #[from]
    #[source]
    E1,
);

#[derive(Debug, thiserror::Error)]
enum E3 {
    #[error("E3::E1:")]
    E1 {
        #[from]
        source: E1,
    },
    #[error("E3::E2:")]
    E2 {
        #[from]
        source: E2,
    },
    #[error("E3::Unknown: {0}")]
    Unknown(
        #[from]
        #[source]
        anyhow::Error,
    ),
}

fn f1() -> Result<(), E1> {
    Err(E1 {
        message: "Hello, Error".to_string(),
    })
}

fn f2() -> Result<(), E2> {
    f1()?;
    Ok(())
}

fn f3() -> Result<(), E3> {
    f2()?;
    Ok(())
}

#[tracing::instrument(err(Debug))]
fn f() -> anyhow::Result<()> {
    tracing::info!("f");
    Ok(f3()?)
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let _ = f();

    Ok(())
}

#[test]
fn test_f3() {
    assert_eq!(f3().unwrap_err().to_string(), "E3::E2:");
    assert_eq!(
        format!("{:?}", f3()),
        "Err(E2 { source: E2(E1 { message: \"Hello, Error\" }) })"
    );
    assert!(f3().unwrap_err().source().is_some());
}
