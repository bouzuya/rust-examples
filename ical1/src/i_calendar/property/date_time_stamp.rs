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

impl DateTimeStamp {
    // TODO: what is value?
    pub fn from_value(value: &str) -> Result<Self, DateTimeStampError> {
        Self::from_string(format!("DTSTAMP:{}\r\n", value))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, DateTimeStampError> {
        if s.starts_with("DTSTAMP:") && s.ends_with("\r\n") {
            let date_time = s
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

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("DTSTAMP:{}\r\n", String::from(self.0))
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
        assert_eq!(DateTimeStamp::from_string(s.clone())?.into_string(), s);

        let s = "DTSTAMP:19971210T080000Z".to_owned();
        assert!(DateTimeStamp::from_string(s).is_err());

        let s = "19971210T080000Z";
        assert_eq!(
            DateTimeStamp::from_value(s)?.into_string(),
            "DTSTAMP:19971210T080000Z\r\n"
        );

        Ok(())
    }
}
