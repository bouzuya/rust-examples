//! Calendar Scale
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.1>

use crate::value_type::{Text, TextError};

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

impl From<CalendarScale> for String {
    fn from(value: CalendarScale) -> String {
        format!("CALSCALE:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for CalendarScale {
    type Error = CalendarScaleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "CALSCALE:GREGORIAN\r\n" {
            Ok(Self(
                Text::try_from("GREGORIAN".to_owned()).map_err(ErrorInner::Text)?,
            ))
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
        assert_fn::<CalendarScale>();

        let s = "CALSCALE:GREGORIAN\r\n".to_owned();
        assert_eq!(String::from(CalendarScale::try_from(s.clone())?), s);

        let s = "CALSCALE:GREGORIAN".to_owned();
        assert!(CalendarScale::try_from(s).is_err());
        Ok(())
    }
}
