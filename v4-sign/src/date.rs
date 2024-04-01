use crate::private::UnixTimestamp;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("timestamp is out of range : {0}")]
    TimestampOutOfRange(i64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Date(u32);

impl Date {
    pub(crate) fn from_unix_timestamp(unix_timestamp: i64) -> Result<Self, Error> {
        Ok(UnixTimestamp::try_from(unix_timestamp)
            .map(|unix_timestamp| Self(unix_timestamp.to_date()))
            .map_err(|_| ErrorKind::TimestampOutOfRange(unix_timestamp))?)
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
}
