#[derive(Debug, thiserror::Error)]
#[error("iana-token")]
pub struct IanaTokenError {
    _private: (),
}

// iana-token    = 1*(ALPHA / DIGIT / "-")
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IanaToken(String);

impl IanaToken {
    pub(in crate::i_calendar) fn from_unescaped(s: &str) -> Result<Self, IanaTokenError> {
        if s.is_empty() || !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            Err(IanaTokenError { _private: () })
        } else {
            Ok(Self(s.to_owned()))
        }
    }

    pub(in crate::i_calendar) fn to_unescaped(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<IanaToken>();

        let s = "IANA-TOKEN";
        assert_eq!(IanaToken::from_unescaped(s)?.to_unescaped(), s);

        let s = "iana-token-1234";
        assert_eq!(IanaToken::from_unescaped(s)?.to_unescaped(), s);

        let s = "";
        assert!(IanaToken::from_unescaped(s).is_err());

        let s = "IANA_TOKEN";
        assert!(IanaToken::from_unescaped(s).is_err());

        Ok(())
    }
}
