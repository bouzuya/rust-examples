/// <https://cloud.google.com/storage/docs/authentication/signatures?hl=ja#credential-scope>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestType {
    Goog4Request,
    Aws4Request,
}

impl RequestType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Goog4Request => "goog4_request",
            Self::Aws4Request => "aws4_request",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use RequestType::*;
        assert_eq!(Goog4Request.as_str(), "goog4_request");
        assert_eq!(Aws4Request.as_str(), "aws4_request");
    }
}
