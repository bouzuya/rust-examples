/// <https://cloud.google.com/storage/docs/authentication/signatures?hl=ja#credential-scope>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Service {
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

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn assert_impls<T: Clone + Copy + std::fmt::Debug + Eq + PartialEq>() {}
        assert_impls::<Service>();

        use Service::*;
        assert_eq!(Storage.as_str(), "storage");
        assert_eq!(format!("{}", Storage), "storage");
        assert_eq!(S3.as_str(), "s3");
        assert_eq!(format!("{}", S3), "s3");
    }
}
