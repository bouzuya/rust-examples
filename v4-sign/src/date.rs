#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("timestamp is out of range : {0}")]
    TimestampOutOfRange(i64),
    #[error("year is out of range (0..=9999) : {0}")]
    YearOutOfRange(i64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Date(u32);

impl Date {
    pub(crate) fn from_unix_timestamp(unix_timestamp: i64) -> Result<Self, Error> {
        // 0000-01-01T00:00:00Z..=9999-12-31T23:59:59Z
        if !(-62_167_219_200..=253_402_300_799).contains(&unix_timestamp) {
            return Err(Error::from(ErrorKind::TimestampOutOfRange(unix_timestamp)));
        }
        let chrono_date_time = chrono::DateTime::from_timestamp(unix_timestamp, 0_u32)
            .ok_or(ErrorKind::TimestampOutOfRange(unix_timestamp))?;
        Self::try_from(chrono_date_time)
    }
}

impl std::convert::TryFrom<chrono::DateTime<chrono::Utc>> for Date {
    type Error = Error;

    fn try_from(value: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let year = chrono::Datelike::year(&value);
        if !(0..=9999).contains(&year) {
            return Err(Error::from(ErrorKind::YearOutOfRange(i64::from(year))));
        }
        let year = year as u32;
        let month = chrono::Datelike::month(&value);
        let day = chrono::Datelike::day(&value);
        let value = year * 10000 + month * 100 + day;
        Ok(Self(value))
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        use anyhow::Context as _;
        let chrono_date_time = chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
            "2020-01-02T03:04:05+00:00",
        )?
        .naive_utc()
        .and_utc();
        let date = Date::try_from(chrono_date_time);
        assert_eq!(date?.to_string(), "20200102");

        let chrono_date_time = chrono::NaiveDate::from_ymd_opt(12000, 1, 2)
            .context("valid naive date")?
            .and_hms_micro_opt(3, 4, 5, 0)
            .context("valid naive date time")?
            .and_utc();
        assert_eq!(chrono_date_time.to_rfc3339(), "+12000-01-02T03:04:05+00:00");
        let date = Date::try_from(chrono_date_time);
        assert!(date.is_err());

        assert!(Date::from_unix_timestamp(-62_167_219_201).is_err());
        assert_eq!(
            Date::from_unix_timestamp(-62_167_219_200)?.to_string(),
            "00000101"
        );
        assert_eq!(Date::from_unix_timestamp(-1)?.to_string(), "19691231");
        assert_eq!(Date::from_unix_timestamp(0)?.to_string(), "19700101");
        assert_eq!(Date::from_unix_timestamp(86399)?.to_string(), "19700101");
        assert_eq!(Date::from_unix_timestamp(86400)?.to_string(), "19700102");
        assert_eq!(
            Date::from_unix_timestamp(253_402_300_799)?.to_string(),
            "99991231"
        );
        assert!(Date::from_unix_timestamp(253_402_300_800).is_err());
        Ok(())
    }
}
