use crate::value_type::{DateTime, DateTimeError};

#[derive(Debug, thiserror::Error)]
#[error("date-time start")]
pub struct DateTimeStartError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("date-time")]
    DateTime(#[from] DateTimeError),
    #[error("invalid format")]
    InvalidFormat,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4>
/// DATE value type not supported
/// dtstparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTimeStart(DateTime);

impl From<DateTimeStart> for String {
    fn from(value: DateTimeStart) -> String {
        format!("DTSTART:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for DateTimeStart {
    type Error = DateTimeStartError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("DTSTART:") && value.ends_with("\r\n") {
            let date_time = value
                .trim_start_matches("DTSTART:")
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
        assert_fn::<DateTimeStart>();

        let s = "DTSTART:19980118T073000Z\r\n".to_owned();
        assert_eq!(String::from(DateTimeStart::try_from(s.clone())?), s);

        let s = "DTSTART:19980118T073000Z".to_owned();
        assert!(DateTimeStart::try_from(s).is_err());
        Ok(())
    }
}
