#[derive(Debug, thiserror::Error)]
#[error("out of range error")]
pub(crate) struct Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UnixTimestamp(i64);

impl UnixTimestamp {
    pub(crate) fn from_rfc3339(s: &str) -> Result<Self, Error> {
        let chrono_date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(s).map_err(|_| Error)?;
        Self::try_from(chrono_date_time.timestamp())
    }

    pub(crate) fn now() -> Self {
        Self::try_from(chrono::Utc::now().timestamp()).expect("current timestamp to be valid")
    }

    pub(crate) fn to_date(self) -> u32 {
        let chrono_date_time =
            chrono::DateTime::from_timestamp(self.0, 0_u32).expect("self.0 to be valid timestamp");
        let year = chrono::Datelike::year(&chrono_date_time) as u32;
        let month = chrono::Datelike::month(&chrono_date_time);
        let day = chrono::Datelike::day(&chrono_date_time);
        year * 10000 + month * 100 + day
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
            Err(Error)
        }
    }
}

#[cfg(test)]
mod tests {
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
}
