/// <https://cloud.google.com/storage/docs/authentication/signatures?hl=ja#credential-scope>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Service {
    Storage,
    S3,
}

impl Service {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Storage => "storage",
            Self::S3 => "s3",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use Service::*;
        assert_eq!(Storage.as_str(), "storage");
        assert_eq!(S3.as_str(), "s3");
    }
}
