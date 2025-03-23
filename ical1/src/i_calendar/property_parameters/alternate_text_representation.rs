//! Alternate Text Representation
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.1>

use crate::i_calendar::value_type::{Uri, UriError};

#[derive(Debug, thiserror::Error)]
#[error("alternate text representation")]
pub struct AlternateTextRepresentationError {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlternateTextRepresentation(Uri);

impl AlternateTextRepresentation {
    pub fn new(value: Uri) -> Result<Self, AlternateTextRepresentationError> {
        Ok(Self(value))
    }

    pub(in crate::i_calendar) fn to_escaped(&self) -> String {
        let mut s = String::new();
        s.push_str("ALTREP=\"");
        s.push_str(&self.0.to_string());
        s.push('"');
        s
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<AlternateTextRepresentation>();

        let s = "CID:part3.msg.970415T083000@example.com";
        assert_eq!(
            AlternateTextRepresentation::new(Uri::from_str(s)?)?.to_escaped(),
            // lower-cased ...
            "ALTREP=\"cid:part3.msg.970415T083000@example.com\""
        );

        Ok(())
    }
}
