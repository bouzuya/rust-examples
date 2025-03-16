use crate::i_calendar::value_type::{DateTime, DateTimeError};

#[derive(Debug, thiserror::Error)]
#[error("date-time end")]
pub struct DateTimeEndError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("date-time")]
    DateTime(#[from] DateTimeError),
    #[error("invalid format")]
    InvalidFormat,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2>
/// DATE value type not supported
/// dtendparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTimeEnd(DateTime);

impl DateTimeEnd {
    // TODO: what is value?
    pub fn from_value(value: &str) -> Result<Self, DateTimeEndError> {
        Self::from_string(format!("DTEND:{}\r\n", value))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, DateTimeEndError> {
        if s.starts_with("DTEND:") && s.ends_with("\r\n") {
            let date_time = s
                .trim_start_matches("DTEND:")
                .trim_end_matches("\r\n")
                .to_owned();
            Ok(DateTime::try_from(date_time)
                .map(Self)
                .map_err(ErrorInner::DateTime)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("DTEND:{}\r\n", String::from(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<DateTimeEnd>();

        let s = "DTEND:19960401T150000Z\r\n".to_owned();
        assert_eq!(DateTimeEnd::from_string(s.clone())?.into_string(), s);

        let s = "DTEND:19960401T150000Z".to_owned();
        assert!(DateTimeEnd::from_string(s).is_err());

        let s = "19960401T150000Z";
        assert_eq!(
            DateTimeEnd::from_value(s)?.into_string(),
            "DTEND:19960401T150000Z\r\n"
        );

        Ok(())
    }
}
