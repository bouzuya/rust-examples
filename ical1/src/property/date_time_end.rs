use crate::value_type::{DateTime, DateTimeError};

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

impl From<DateTimeEnd> for String {
    fn from(value: DateTimeEnd) -> String {
        format!("DTEND:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for DateTimeEnd {
    type Error = DateTimeEndError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("DTEND:") && value.ends_with("\r\n") {
            let date_time = value
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<DateTimeEnd>();

        let s = "DTEND:19960401T150000Z\r\n".to_owned();
        assert_eq!(String::from(DateTimeEnd::try_from(s.clone())?), s);

        let s = "DTEND:19960401T150000Z".to_owned();
        assert!(DateTimeEnd::try_from(s).is_err());
        Ok(())
    }
}
