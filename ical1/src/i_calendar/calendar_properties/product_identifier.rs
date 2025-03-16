//! Product Identifier
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.3>

use crate::i_calendar::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("unique identifier")]
pub struct ProductIdentifierError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("invalid format")]
    InvalidFormat,
    #[error("text")]
    Text(#[from] TextError),
}

/// pidparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProductIdentifier(Text);

impl ProductIdentifier {
    // TODO: what is value?
    pub fn from_value(s: &str) -> Result<Self, ProductIdentifierError> {
        Self::from_string(format!("PRODID:{}\r\n", s))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, ProductIdentifierError> {
        if s.starts_with("PRODID:") && s.ends_with("\r\n") {
            let text = s
                .trim_start_matches("PRODID:")
                .trim_end_matches("\r\n")
                .to_owned();
            Ok(Text::try_from(text).map(Self).map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("PRODID:{}\r\n", String::from(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<ProductIdentifier>();

        let s = "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n".to_owned();
        assert_eq!(ProductIdentifier::from_string(s.clone())?.into_string(), s);

        let s = "PRODID:-//ABC Corporation//NONSGML My Product//EN".to_owned();
        assert!(ProductIdentifier::from_string(s.clone()).is_err());

        let s = "-//ABC Corporation//NONSGML My Product//EN";
        assert_eq!(
            ProductIdentifier::from_value(s)?.into_string(),
            "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n"
        );

        Ok(())
    }
}
