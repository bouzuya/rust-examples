//! Version
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.4>

use crate::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("version")]
pub struct VersionError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
    #[error("invalid format")]
    InvalidFormat,
}

/// verparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version(Text);

impl From<Version> for String {
    fn from(value: Version) -> String {
        format!("VERSION:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for Version {
    type Error = VersionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("VERSION:") && value.ends_with("\r\n") {
            Ok(Text::try_from(
                value
                    .trim_start_matches("VERSION:")
                    .trim_end_matches("\r\n")
                    .to_owned(),
            )
            .map(Self)
            .map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Version>();

        let s = "VERSION:2.0\r\n".to_owned();
        assert_eq!(String::from(Version::try_from(s.clone())?), s);

        let s = "VERSION:2.0".to_owned();
        assert!(Version::try_from(s).is_err());
        Ok(())
    }
}
