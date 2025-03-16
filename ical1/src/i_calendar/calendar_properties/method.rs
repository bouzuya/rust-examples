//! Method
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.2>

use crate::i_calendar::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("method")]
pub struct MethodError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
    #[error("invalid format")]
    InvalidFormat,
}

/// metparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Method(Text);

impl Method {
    // TODO: what is value?
    pub fn from_value(s: &str) -> Result<Self, MethodError> {
        Self::from_string(format!("METHOD:{}\r\n", s))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, MethodError> {
        if s.starts_with("METHOD:") && s.ends_with("\r\n") {
            Ok(Text::try_from(
                s.trim_start_matches("METHOD:")
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
        format!("METHOD:{}\r\n", String::from(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Method>();

        let s = "METHOD:REQUEST\r\n".to_owned();
        assert_eq!(Method::from_string(s.clone())?.into_string(), s);

        let s = "METHOD:REQUEST".to_owned();
        assert!(Method::from_string(s).is_err());

        let s = "REQUEST";
        assert_eq!(Method::from_value(s)?.into_string(), "METHOD:REQUEST\r\n");

        Ok(())
    }
}
