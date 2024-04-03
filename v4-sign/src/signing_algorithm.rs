/// <https://cloud.google.com/storage/docs/authentication/signatures#signing_algorithm>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SigningAlgorithm {
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

impl std::convert::AsRef<str> for SigningAlgorithm {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use SigningAlgorithm::*;

        fn assert_impls<T: AsRef<str>>() {}
        assert_impls::<SigningAlgorithm>();

        assert_eq!(Goog4RsaSha256.as_str(), "GOOG4-RSA-SHA256");
        assert_eq!(Goog4HmacSha256.as_str(), "GOOG4-HMAC-SHA256");
        assert_eq!(Aws4HmacSha256.as_str(), "AWS4-HMAC-SHA256");

        assert_eq!(Goog4RsaSha256.as_ref(), "GOOG4-RSA-SHA256");
        assert_eq!(Goog4HmacSha256.as_ref(), "GOOG4-HMAC-SHA256");
        assert_eq!(Aws4HmacSha256.as_ref(), "AWS4-HMAC-SHA256");
    }
}
