#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Debug, thiserror::Error)]
enum ErrorKind {
    #[error("out of range (0..=604800) : {0}")]
    OutOfRange(i64),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Expiration(i64);

impl std::convert::TryFrom<i64> for Expiration {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (0..=604_800).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::from(ErrorKind::OutOfRange(value)))
        }
    }
}

impl std::fmt::Display for Expiration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert!(Expiration::try_from(-1_i64).is_err());
        assert!(Expiration::try_from(0_i64).is_ok());
        let expiration = Expiration::try_from(604_800_i64)?;
        assert_eq!(expiration.to_string(), "604800");
        assert!(Expiration::try_from(604_801_i64).is_err());
        Ok(())
    }
}
