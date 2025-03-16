//! Calendar Scale
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.1>

use crate::i_calendar::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("calendar scale")]
pub struct CalendarScaleError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
    #[error("invalid format")]
    InvalidFormat,
}

/// calparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CalendarScale(Text);

impl CalendarScale {
    // TODO: what is value?
    pub fn from_value(s: &str) -> Result<Self, CalendarScaleError> {
        Self::from_string(format!("CALSCALE:{}\r\n", s))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, CalendarScaleError> {
        if s == "CALSCALE:GREGORIAN\r\n" {
            Ok(Self(
                Text::from_string("GREGORIAN".to_owned()).map_err(ErrorInner::Text)?,
            ))
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("CALSCALE:{}\r\n", self.0.into_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<CalendarScale>();

        let s = "CALSCALE:GREGORIAN\r\n".to_owned();
        assert_eq!(CalendarScale::from_string(s.clone())?.into_string(), s);

        let s = "CALSCALE:GREGORIAN".to_owned();
        assert!(CalendarScale::from_string(s).is_err());

        let s = "GREGORIAN";
        assert_eq!(
            CalendarScale::from_value(s)?.into_string(),
            "CALSCALE:GREGORIAN\r\n"
        );

        Ok(())
    }
}
