//! Version
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.4>

use crate::i_calendar::value_type::{Text, TextError};

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

impl Version {
    // TODO: what is value?
    pub fn from_value(s: &str) -> Result<Self, VersionError> {
        Self::from_string(format!("VERSION:{}\r\n", s))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, VersionError> {
        if s.starts_with("VERSION:") && s.ends_with("\r\n") {
            Ok(Text::from_string(
                s.trim_start_matches("VERSION:")
                    .trim_end_matches("\r\n")
                    .to_owned(),
            )
            .map(Self)
            .map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("VERSION:{}\r\n", self.0.into_string())
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
        assert_eq!(Version::from_string(s.clone())?.into_string(), s);

        let s = "VERSION:2.0".to_owned();
        assert!(Version::from_string(s).is_err());

        let s = "2.0";
        assert_eq!(Version::from_value(s)?.into_string(), "VERSION:2.0\r\n");

        Ok(())
    }
}
