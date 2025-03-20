//! Version
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.4>

use crate::i_calendar::{
    property_parameters::OtherParam,
    value_type::{Text, TextError},
};

#[derive(Debug, thiserror::Error)]
#[error("version")]
pub struct VersionError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("invalid format")]
    InvalidFormat,
    #[error("text")]
    Text(#[from] TextError),
}

/// verparam not supported
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version(Text, Vec<OtherParam>);

impl Version {
    pub fn new(value: Text) -> Result<Self, VersionError> {
        Self::with_parameters(value, Vec::<OtherParam>::new())
    }

    pub fn with_parameters<I>(value: Text, param: I) -> Result<Self, VersionError>
    where
        I: IntoIterator,
        I::Item: Into<OtherParam>,
    {
        Ok(Self(
            value,
            param
                .into_iter()
                .map(Into::into)
                .collect::<Vec<OtherParam>>(),
        ))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, VersionError> {
        // TODO: support parameters
        if s.starts_with("VERSION:") && s.ends_with("\r\n") {
            Ok(Text::from_string(
                s.trim_start_matches("VERSION:")
                    .trim_end_matches("\r\n")
                    .to_owned(),
            )
            .map(|it| Self(it, Vec::new()))
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
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<Version>();

        let s = "VERSION:2.0\r\n".to_owned();
        assert_eq!(Version::from_string(s.clone())?.into_string(), s);

        let s = "VERSION:2.0".to_owned();
        assert!(Version::from_string(s).is_err());

        assert_eq!(
            Version::new(Text::from_unescaped("2.0")?)?.into_string(),
            "VERSION:2.0\r\n"
        );

        Ok(())
    }
}
