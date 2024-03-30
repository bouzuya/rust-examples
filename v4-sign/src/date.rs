use chrono::Datelike;

#[derive(Debug, thiserror::Error)]
#[error("year is out of range (0..=9999) : {0}")]
pub(crate) struct Error(i64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Date(u32);

impl std::convert::TryFrom<chrono::DateTime<chrono::Utc>> for Date {
    type Error = Error;

    fn try_from(value: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let year = value.year();
        if !(0..=9999).contains(&year) {
            return Err(Error(i64::from(year)));
        }
        let year = year as u32;
        let month = value.month();
        let day = value.day();
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
        Ok(())
    }
}
