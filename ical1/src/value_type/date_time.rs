#[derive(Debug, thiserror::Error)]
#[error("date-time")]
pub struct DateTimeError {
    // TODO: improve error message
    _private: (),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5>
/// FORM #1 and FORM #3 are not supported
/// YYYYMMDDTHHMMSSZ
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTime(String);

impl TryFrom<String> for DateTime {
    type Error = DateTimeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value
            .chars()
            .all(|c| c.is_ascii_digit() || c == 'T' || c == 'Z')
            && value.ends_with('Z')
        {
            // TODO: improve validation
            Ok(Self(value))
        } else {
            Err(DateTimeError { _private: () })
        }
    }
}

impl From<DateTime> for String {
    fn from(value: DateTime) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<DateTime>();

        // UTC time
        let s = "19970714T173000Z".to_owned();
        assert_eq!(String::from(DateTime::try_from(s.clone())?), s);

        // Local time
        let s = "19970714T133000".to_owned();
        assert!(DateTime::try_from(s).is_err());

        // Local time with time zone reference
        let s = "TZID=America/New_York:19970714T133000".to_owned();
        assert!(DateTime::try_from(s).is_err());
        Ok(())
    }
}
