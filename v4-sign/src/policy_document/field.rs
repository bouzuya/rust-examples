#[derive(Debug, thiserror::Error)]
#[error("unknown field: {0}")]
pub struct Error(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field(Inner);

impl Field {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, Error> {
        Inner::new(s).map(Field)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Inner {
    Acl,
    Bucket,
    CacheControl,
    ContentDisposition,
    ContentEncoding,
    ContentLength,
    ContentType,
    Expires,
    File,
    Key,
    Policy,
    SuccessActionRedirect,
    SuccessActionStatus,
    XGoogAlgorithm,
    XGoogCredential,
    XGoogCustomTime,
    XGoogDate,
    XGoogSignature,
    XGoogMeta(String),
}

impl Inner {
    fn new<S: AsRef<str>>(s: S) -> Result<Self, Error> {
        match s.as_ref() {
            "acl" => Ok(Self::Acl),
            "bucket" => Ok(Self::Bucket),
            "Cache-Control" => Ok(Self::CacheControl),
            "Content-Disposition" => Ok(Self::ContentDisposition),
            "Content-Encoding" => Ok(Self::ContentEncoding),
            "Content-Length" => Ok(Self::ContentLength),
            "Content-Type" => Ok(Self::ContentType),
            "Expires" => Ok(Self::Expires),
            "file" => Ok(Self::File),
            "key" => Ok(Self::Key),
            "policy" => Ok(Self::Policy),
            "success_action_redirect" => Ok(Self::SuccessActionRedirect),
            "success_action_status" => Ok(Self::SuccessActionStatus),
            "x-goog-algorithm" => Ok(Self::XGoogAlgorithm),
            "x-goog-credential" => Ok(Self::XGoogCredential),
            "x-goog-custom-time" => Ok(Self::XGoogCustomTime),
            "x-goog-date" => Ok(Self::XGoogDate),
            "x-goog-signature" => Ok(Self::XGoogSignature),
            s => match s.strip_prefix("x-goog-meta-") {
                None => Err(Error(s.to_string())),
                Some(custom_metadata) => {
                    if custom_metadata.is_empty() {
                        Err(Error(s.to_string()))
                    } else {
                        Ok(Self::XGoogMeta(custom_metadata.to_string()))
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_impls<T: Clone + Eq + PartialEq>() {}
        assert_impls::<Field>();

        for (field, inner) in [
            ("acl", Inner::Acl),
            ("bucket", Inner::Bucket),
            ("Cache-Control", Inner::CacheControl),
            ("Content-Disposition", Inner::ContentDisposition),
            ("Content-Encoding", Inner::ContentEncoding),
            ("Content-Length", Inner::ContentLength),
            ("Content-Type", Inner::ContentType),
            ("Expires", Inner::Expires),
            ("file", Inner::File),
            ("key", Inner::Key),
            ("policy", Inner::Policy),
            ("success_action_redirect", Inner::SuccessActionRedirect),
            ("success_action_status", Inner::SuccessActionStatus),
            ("x-goog-algorithm", Inner::XGoogAlgorithm),
            ("x-goog-credential", Inner::XGoogCredential),
            ("x-goog-custom-time", Inner::XGoogCustomTime),
            ("x-goog-date", Inner::XGoogDate),
            ("x-goog-signature", Inner::XGoogSignature),
            (
                "x-goog-meta-reviewer",
                Inner::XGoogMeta("reviewer".to_string()),
            ),
        ] {
            assert_eq!(Field::new(field)?, Field(inner.clone()));
        }

        assert_eq!(Field::new("").unwrap_err().to_string(), "unknown field: ");
        assert_eq!(
            Field::new("Acl").unwrap_err().to_string(),
            "unknown field: Acl"
        );
        assert_eq!(
            Field::new("x-goog-meta-").unwrap_err().to_string(),
            "unknown field: x-goog-meta-"
        );
        Ok(())
    }
}
