/// <https://cloud.google.com/storage/docs/authentication/signatures?hl=ja#credential-scope>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestType {
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

impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn assert_impls<T: Clone + Copy + std::fmt::Debug + std::fmt::Display + Eq + PartialEq>() {}
        assert_impls::<RequestType>();

        use RequestType::*;
        assert_eq!(Goog4Request.as_str(), "goog4_request");
        assert_eq!(format!("{}", Goog4Request), "goog4_request");
        assert_eq!(Aws4Request.as_str(), "aws4_request");
        assert_eq!(format!("{}", Aws4Request), "aws4_request");
    }
}
