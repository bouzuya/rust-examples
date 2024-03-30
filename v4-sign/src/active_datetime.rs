#[derive(Debug, thiserror::Error)]
#[error("error")]
pub struct Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ActiveDatetime(i64);

impl ActiveDatetime {
    pub(crate) fn from_unix_timestamp(unix_timestamp: i64) -> Result<Self, Error> {
        let chrono_date_time =
            chrono::DateTime::from_timestamp(unix_timestamp, 0_u32).ok_or(Error)?;
        Self::try_from(chrono_date_time)
    }

    pub(crate) fn now() -> Self {
        Self(chrono::Utc::now().timestamp())
    }

    pub(crate) fn unix_timestamp(&self) -> i64 {
        self.0
    }
}

impl std::convert::TryFrom<chrono::DateTime<chrono::Utc>> for ActiveDatetime {
    type Error = Error;

    fn try_from(value: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let year = chrono::Datelike::year(&value);
        if !(0..=9999).contains(&year) {
            return Err(Error);
        }
        Ok(Self(value.timestamp()))
    }
}

impl std::fmt::Display for ActiveDatetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let chrono_date_time =
            chrono::DateTime::from_timestamp(self.0, 0_u32).expect("self.0 to be valid timestamp");
        let s = chrono_date_time.format("%Y%m%dT%H%M%SZ");
        s.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let chrono_date_time = chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339(
            "2020-01-02T03:04:05+00:00",
        )?
        .naive_utc()
        .and_utc();
        assert_eq!(
            ActiveDatetime::try_from(chrono_date_time)?.to_string(),
            "20200102T030405Z"
        );

        let unix_timestamp = chrono_date_time.timestamp();
        let active_datetime = ActiveDatetime::from_unix_timestamp(unix_timestamp)?;
        assert_eq!(active_datetime.unix_timestamp(), unix_timestamp);

        Ok(())
    }
}
