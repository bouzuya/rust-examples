//! URI
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.13>

#[derive(Debug, PartialEq)]
pub struct Uri(url::Url);

#[derive(Debug, thiserror::Error)]
#[error("uri")]
pub struct UriError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("invalid uri")]
    InvalidUri(#[source] url::ParseError),
}

impl std::str::FromStr for Uri {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(url::Url::parse(s)
            .map(Uri)
            .map_err(ErrorInner::InvalidUri)?)
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test_valid_uri() -> anyhow::Result<()> {
        let s = "http://example.com/my-report.txt";
        let uri = Uri::from_str(s)?;
        assert_eq!(uri.to_string(), s);
        Ok(())
    }

    #[test]
    fn test_invalid_uri() -> anyhow::Result<()> {
        let s = "invalid_uri";
        let result = Uri::from_str(s);
        assert!(result.is_err());
        Ok(())
    }
}
