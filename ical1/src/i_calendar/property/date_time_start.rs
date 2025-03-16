use crate::i_calendar::value_type::{DateTime, DateTimeError};

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

impl DateTimeStart {
    // TODO: what is value?
    pub fn from_value(value: &str) -> Result<Self, DateTimeStartError> {
        Self::from_string(format!("DTSTART:{}\r\n", value))
    }

    pub(in crate::i_calendar) fn from_string(value: String) -> Result<Self, DateTimeStartError> {
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

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("DTSTART:{}\r\n", String::from(self.0))
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
        assert_eq!(DateTimeStart::from_string(s.clone())?.into_string(), s);

        let s = "DTSTART:19980118T073000Z".to_owned();
        assert!(DateTimeStart::from_string(s).is_err());

        let s = "19980118T073000Z";
        assert_eq!(
            DateTimeStart::from_value(s)?.into_string(),
            "DTSTART:19980118T073000Z\r\n"
        );
        Ok(())
    }
}
