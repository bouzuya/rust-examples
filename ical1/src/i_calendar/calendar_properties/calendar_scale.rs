//! Calendar Scale
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.1>

use crate::i_calendar::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("calendar scale")]
pub struct CalendarScaleError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("invalid calvalue")]
    InvalidCalvalue,
    #[error("invalid format")]
    InvalidFormat,
    #[error("text")]
    Text(#[from] TextError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarScale {
    _private: (),
}

impl CalendarScale {
    pub fn new(text: Text) -> Result<Self, CalendarScaleError> {
        if text.to_string() != "GREGORIAN" {
            return Err(ErrorInner::InvalidCalvalue)?;
        }
        Ok(Self { _private: () })
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, CalendarScaleError> {
        if s == "CALSCALE:GREGORIAN\r\n" {
            Ok(Self { _private: () })
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        "CALSCALE:GREGORIAN\r\n".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<CalendarScale>();

        let s = "CALSCALE:GREGORIAN\r\n".to_owned();
        assert_eq!(CalendarScale::from_string(s.clone())?.into_string(), s);

        let s = "CALSCALE:GREGORIAN".to_owned();
        assert!(CalendarScale::from_string(s).is_err());

        let s = "GREGORIAN";
        assert_eq!(
            CalendarScale::new(Text::from_unescaped(s)?)?.into_string(),
            "CALSCALE:GREGORIAN\r\n"
        );

        Ok(())
    }
}
