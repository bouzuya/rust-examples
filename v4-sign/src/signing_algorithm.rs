/// <https://cloud.google.com/storage/docs/authentication/signatures#signing_algorithm>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SigningAlgorithm {
    Goog4RsaSha256,
    Goog4HmacSha256,
    Aws4HmacSha256,
}

impl SigningAlgorithm {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Goog4RsaSha256 => "GOOG4-RSA-SHA256",
            Self::Goog4HmacSha256 => "GOOG4-HMAC-SHA256",
            Self::Aws4HmacSha256 => "AWS4-HMAC-SHA256",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use SigningAlgorithm::*;
        assert_eq!(Goog4RsaSha256.as_str(), "GOOG4-RSA-SHA256");
        assert_eq!(Goog4HmacSha256.as_str(), "GOOG4-HMAC-SHA256");
        assert_eq!(Aws4HmacSha256.as_str(), "AWS4-HMAC-SHA256");
    }
}
