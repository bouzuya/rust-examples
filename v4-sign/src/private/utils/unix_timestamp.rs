use std::time::SystemTime;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("invalid iso8601 format: {0}")]
    InvalidIso8601Format(String),
    #[error("invalid rfc3339 format: {0}")]
    InvalidRfc3339Format(String),
    #[error("out of range: {0}")]
    OutOfRange(i64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UnixTimestamp(i64);

impl UnixTimestamp {
    pub(crate) fn from_iso8601_basic_format_date_time(s: &str) -> Result<Self, Error> {
        let chrono_date_time = chrono::NaiveDateTime::parse_from_str(s, "%Y%m%dT%H%M%SZ")
            .map_err(|_| ErrorKind::InvalidIso8601Format(s.to_string()))?
            .and_utc();
        Self::try_from(chrono_date_time.timestamp())
    }

    pub(crate) fn from_rfc3339(s: &str) -> Result<Self, Error> {
        let chrono_date_time = chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(s)
            .map_err(|_| ErrorKind::InvalidRfc3339Format(s.to_string()))?;
        Self::try_from(chrono_date_time.timestamp())
    }

    pub(crate) fn from_system_time(system_time: SystemTime) -> Result<Self, Error> {
        Self::try_from(
            i64::try_from(
                system_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|_| ErrorKind::OutOfRange(0))?
                    .as_secs(),
            )
            .map_err(|_| ErrorKind::OutOfRange(0))?,
        )
    }

    pub(crate) fn to_date(self) -> u32 {
        let chrono_date_time =
            chrono::DateTime::from_timestamp(self.0, 0_u32).expect("self.0 to be valid timestamp");
        let year = chrono::Datelike::year(&chrono_date_time) as u32;
        let month = chrono::Datelike::month(&chrono_date_time);
        let day = chrono::Datelike::day(&chrono_date_time);
        year * 10000 + month * 100 + day
    }

    pub(crate) fn to_rfc3339(self) -> String {
        let chrono_date_time =
            chrono::DateTime::from_timestamp(self.0, 0_u32).expect("self.0 to be valid timestamp");
        chrono_date_time.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    pub(crate) fn to_iso8601_basic_format_date_time(self) -> String {
        let chrono_date_time =
            chrono::DateTime::from_timestamp(self.0, 0_u32).expect("self.0 to be valid timestamp");
        chrono_date_time.format("%Y%m%dT%H%M%SZ").to_string()
    }
}

impl std::convert::From<UnixTimestamp> for i64 {
    fn from(value: UnixTimestamp) -> Self {
        value.0
    }
}

impl std::convert::TryFrom<i64> for UnixTimestamp {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        // 0000-01-01T00:00:00Z..=9999-12-31T23:59:59Z
        if (-62_167_219_200..=253_402_300_799).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ErrorKind::OutOfRange(value))?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chrono() -> anyhow::Result<()> {
        let chrono_date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2020-01-02T03:04:05Z")?
                .naive_utc()
                .and_utc();
        let date = chrono_date_time.format("%Y%m%d").to_string();
        let x_goog_date = chrono_date_time.format("%Y%m%dT%H%M%SZ").to_string();
        assert_eq!(date, "20200102");
        assert_eq!(x_goog_date, "20200102T030405Z");
        Ok(())
    }

    #[test]
    fn test_convert_i64() -> anyhow::Result<()> {
        for x in [-62_167_219_200_i64, 0_i64, 253_402_300_799_i64] {
            assert_eq!(i64::from(UnixTimestamp::try_from(x)?), x);
        }
        for x in [-62_167_219_200_i64 - 1, 253_402_300_799_i64 + 1] {
            assert_eq!(
                UnixTimestamp::try_from(x).unwrap_err().to_string(),
                format!("out of range: {}", x)
            );
        }
        Ok(())
    }

    #[test]
    fn test_convert_iso8601_basic_format_date_time() -> anyhow::Result<()> {
        for s in ["00000101T000000Z", "20200102T030405Z", "99991231T235959Z"] {
            assert_eq!(
                UnixTimestamp::from_iso8601_basic_format_date_time(s)?
                    .to_iso8601_basic_format_date_time(),
                s
            );
        }
        assert_eq!(
            UnixTimestamp::from_iso8601_basic_format_date_time("+100000101T000000Z")
                .unwrap_err()
                .to_string(),
            "invalid iso8601 format: +100000101T000000Z"
        );
        Ok(())
    }

    #[test]
    fn test_convert_rfc3339() -> anyhow::Result<()> {
        for s in [
            "0000-01-01T00:00:00Z",
            "2020-01-02T03:04:05Z",
            "9999-12-31T23:59:59Z",
        ] {
            assert_eq!(UnixTimestamp::from_rfc3339(s)?.to_rfc3339(), s);
        }
        assert_eq!(
            UnixTimestamp::from_rfc3339("+10000-01-01T00:00:00Z")
                .unwrap_err()
                .to_string(),
            "invalid rfc3339 format: +10000-01-01T00:00:00Z"
        );
        Ok(())
    }
}
