use std::str::FromStr as _;

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("hello");
    tracing::error!("1 {}", Error::NoSource);
    tracing::error!("2 {:?}", Error::NoSource);
    tracing::error!(
        "3 {:?}",
        Error::SourceLevel2(Error2::SourceLevel1(i32::from_str("abc").unwrap_err()))
    );
    tracing::error!(
        "4 {:?}",
        anyhow::anyhow!(Error::SourceLevel2(Error2::SourceLevel1(
            i32::from_str("abc").unwrap_err()
        )))
    );
    tracing::error!(
        "5 {:?}",
        anyhow::anyhow!(Error::Boxed(
            Error2::SourceLevel1(i32::from_str("abc").unwrap_err()).into()
        ))
    );
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no source")]
    NoSource,
    #[error("source level 1")]
    SourceLevel1(#[source] std::num::ParseIntError),
    #[error("source level 2")]
    SourceLevel2(#[source] Error2),
    #[error("boxed")]
    Boxed(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error2 {
    #[error("source level 1'")]
    SourceLevel1(#[source] std::num::ParseIntError),
}

#[derive(Debug, thiserror::Error)]
#[error("error3")]
pub struct Error3(#[source] Box<dyn std::error::Error + Send + Sync>);

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test() {
        let e = std::io::Error::new(std::io::ErrorKind::Other, "other error");
        assert_eq!(format!("{}", e), "other error");
        assert_eq!(
            format!("{:?}", e),
            "Custom { kind: Other, error: \"other error\" }"
        );

        let e = Error::NoSource;
        assert_eq!(format!("{}", e), "no source");
        assert_eq!(format!("{:?}", e), "NoSource");

        let e = Error::SourceLevel1(i32::from_str("abc").unwrap_err());
        assert_eq!(format!("{}", e), "source level 1");
        assert_eq!(
            format!("{:?}", e),
            "SourceLevel1(ParseIntError { kind: InvalidDigit })"
        );

        let e = anyhow::anyhow!(Error::SourceLevel1(i32::from_str("abc").unwrap_err()));
        assert_eq!(format!("{}", e), "source level 1");
        assert_eq!(
            format!("{:?}", e),
            [
                "source level 1",
                "",
                "Caused by:",
                "    invalid digit found in string",
            ]
            .join("\n")
        );

        let e = anyhow::anyhow!(Error::SourceLevel2(Error2::SourceLevel1(
            i32::from_str("abc").unwrap_err()
        )));
        assert_eq!(format!("{}", e), "source level 2");
        assert_eq!(
            format!("{:?}", e),
            [
                "source level 2",
                "",
                "Caused by:",
                "    0: source level 1'",
                "    1: invalid digit found in string",
            ]
            .join("\n")
        );

        let e = anyhow::anyhow!(Error::Boxed(
            Error2::SourceLevel1(i32::from_str("abc").unwrap_err()).into()
        ));
        assert_eq!(format!("{}", e), "boxed");
        assert_eq!(
            format!("{:?}", e),
            [
                "boxed",
                "",
                "Caused by:",
                "    0: source level 1'",
                "    1: invalid digit found in string",
            ]
            .join("\n")
        );

        let e = Error3(Error2::SourceLevel1(i32::from_str("abc").unwrap_err()).into());
        assert_eq!(format!("{}", e), "error3");
        assert_eq!(
            format!("{:?}", e),
            "Error3(SourceLevel1(ParseIntError { kind: InvalidDigit }))"
        );
        assert_eq!(
            format!("{:?}", anyhow::anyhow!(e)),
            [
                "error3",
                "",
                "Caused by:",
                "    0: source level 1'",
                "    1: invalid digit found in string",
            ]
            .join("\n")
        );
    }
}
