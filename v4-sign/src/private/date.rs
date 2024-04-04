use crate::private::utils::UnixTimestamp;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("invalid format : {0}")]
    InvalidFormat(String),
    #[error("timestamp is out of range : {0}")]
    TimestampOutOfRange(i64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Date(u32);

impl Date {
    pub(crate) fn from_unix_timestamp_obj(unix_timestamp: UnixTimestamp) -> Self {
        Self(unix_timestamp.to_date())
    }

    pub fn from_unix_timestamp(unix_timestamp: i64) -> Result<Self, Error> {
        Ok(UnixTimestamp::try_from(unix_timestamp)
            .map(|unix_timestamp| Self(unix_timestamp.to_date()))
            .map_err(|_| ErrorKind::TimestampOutOfRange(unix_timestamp))?)
    }
}

impl std::convert::TryFrom<&str> for Date {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            return Err(ErrorKind::InvalidFormat(value.to_string()))?;
        }
        let yyyymmdd = value
            .parse::<u32>()
            .map_err(|_| ErrorKind::InvalidFormat(value.to_string()))?;
        let yyyy = yyyymmdd / 10000;
        if !(0..=9999).contains(&yyyy) {
            return Err(ErrorKind::InvalidFormat(value.to_string()))?;
        }
        let mm = (yyyymmdd % 10000) / 100;
        if !(1..=12).contains(&mm) {
            return Err(ErrorKind::InvalidFormat(value.to_string()))?;
        }
        let dd = yyyymmdd % 100;
        if !(1..=31).contains(&dd) {
            return Err(ErrorKind::InvalidFormat(value.to_string()))?;
        }
        // FIXME: validate date
        Ok(Self(yyyy * 10000 + mm * 100 + dd))
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
        let min_unix_timestamp =
            i64::from(UnixTimestamp::from_rfc3339("0000-01-01T00:00:00+00:00")?);
        assert!(Date::from_unix_timestamp(min_unix_timestamp - 1).is_err());
        assert!(Date::from_unix_timestamp(min_unix_timestamp).is_ok());

        let unix_timestamp = i64::from(UnixTimestamp::from_rfc3339("2020-01-02T03:04:05+00:00")?);
        let date = Date::from_unix_timestamp(unix_timestamp);
        assert_eq!(date?.to_string(), "20200102");

        let max_unix_timestamp =
            i64::from(UnixTimestamp::from_rfc3339("9999-12-31T23:59:59+00:00")?);
        assert!(Date::from_unix_timestamp(max_unix_timestamp).is_ok());
        assert!(Date::from_unix_timestamp(max_unix_timestamp + 1).is_err());

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

    #[test]
    fn test_try_from_str() -> anyhow::Result<()> {
        assert_eq!(Date::try_from("00000101")?.to_string(), "00000101");
        assert_eq!(Date::try_from("20200102")?.to_string(), "20200102");
        assert_eq!(Date::try_from("99991231")?.to_string(), "99991231");
        Ok(())
    }
}
