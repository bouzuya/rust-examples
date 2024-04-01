use std::str::FromStr;

use crate::private::UnixTimestamp;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    // TODO:
    #[error("invalid format or out of range")]
    InvalidFormatOrOutOfRange(#[from] crate::private::Error),
}

// YYYYMMDD'T'HHMMSS'Z'
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Expiration(UnixTimestamp);

impl std::fmt::Display for Expiration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.to_iso8601_basic_format_date_time().fmt(f)
    }
}

impl std::str::FromStr for Expiration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unix_timestamp =
            UnixTimestamp::from_iso8601_basic_format_date_time(s).map_err(ErrorKind::from)?;
        Ok(Expiration(unix_timestamp))
    }
}

impl<'de> serde::Deserialize<'de> for Expiration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Expiration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string in the format of YYYYMMDD'T'HHMMSS'Z'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Self::Value::from_str(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_impls<
            T: Clone
                + Copy
                + std::fmt::Debug
                + Eq
                + PartialEq
                + std::str::FromStr
                + serde::Deserialize<'static>,
        >() {
        }
        assert_impls::<Expiration>();

        let s = "20200616T111111Z";
        let expiration = Expiration::from_str(s).unwrap();
        assert_eq!(expiration.to_string(), s);
        assert_eq!(
            serde_json::from_str::<Expiration>(&format!("\"{}\"", s))?,
            expiration
        );
        Ok(())
    }
}
