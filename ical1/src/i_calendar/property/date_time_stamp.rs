use crate::i_calendar::value_type::{DateTime, DateTimeError};

#[derive(Debug, thiserror::Error)]
#[error("date-time stamp")]
pub struct DateTimeStampError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("date-time")]
    DateTime(#[source] DateTimeError),
    #[error("invalid format")]
    InvalidFormat,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2>
/// stmparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTimeStamp(DateTime);

impl From<DateTimeStamp> for String {
    fn from(value: DateTimeStamp) -> String {
        format!("DTSTAMP:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for DateTimeStamp {
    type Error = DateTimeStampError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("DTSTAMP:") && value.ends_with("\r\n") {
            let date_time = value
                .trim_start_matches("DTSTAMP:")
                .trim_end_matches("\r\n")
                .to_owned();
            Ok(DateTime::try_from(date_time)
                .map(Self)
                .map_err(ErrorInner::DateTime)?)
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
        assert_fn::<DateTimeStamp>();

        let s = "DTSTAMP:19971210T080000Z\r\n".to_owned();
        assert_eq!(String::from(DateTimeStamp::try_from(s.clone())?), s);

        let s = "DTSTAMP:19971210T080000Z".to_owned();
        assert!(DateTimeStamp::try_from(s).is_err());
        Ok(())
    }
}
