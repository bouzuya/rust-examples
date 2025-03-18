#[derive(Debug, thiserror::Error)]
#[error("x-name")]
pub struct XNameError {
    _private: (),
}

// x-name        = "X-" [vendorid "-"] 1*(ALPHA / DIGIT / "-")
// ; Reserved for experimental use.
// vendorid      = 3*(ALPHA / DIGIT)
// ; Vendor identification
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct XName(String);

impl XName {
    pub(in crate::i_calendar) fn from_unescaped(s: &str) -> Result<Self, XNameError> {
        if !s.starts_with("X-") || !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            Err(XNameError { _private: () })
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
        assert_fn::<XName>();

        let s = "X-VEN-ABC123";
        assert_eq!(XName::from_unescaped(s)?.to_unescaped(), s);

        let s = "X-WR-CALNAME";
        assert_eq!(XName::from_unescaped(s)?.to_unescaped(), s);

        let s = "WR-CALNAME";
        assert!(XName::from_unescaped(s).is_err());

        let s = "X-WR_CALNAME";
        assert!(XName::from_unescaped(s).is_err());

        Ok(())
    }
}
