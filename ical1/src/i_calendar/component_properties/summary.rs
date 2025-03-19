use crate::i_calendar::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("summary")]
pub struct SummaryError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
    #[error("invalid format")]
    InvalidFormat,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.12>
/// summparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Summary(Text);

impl Summary {
    // TODO: what is value?
    pub fn from_value(value: &str) -> Result<Self, SummaryError> {
        Self::from_string(format!("SUMMARY:{}\r\n", value))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, SummaryError> {
        if s.starts_with("SUMMARY:") && s.ends_with("\r\n") {
            let text = s
                .trim_start_matches("SUMMARY:")
                .trim_end_matches("\r\n")
                .to_owned();
            Ok(Text::from_string(text)
                .map(Self)
                .map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("SUMMARY:{}\r\n", self.0.into_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Summary>();

        let s = "SUMMARY:Department Party\r\n".to_owned();
        assert_eq!(Summary::from_string(s.clone())?.into_string(), s);

        let s = "SUMMARY:Department Party".to_owned();
        assert!(Summary::from_string(s).is_err());

        let s = "Department Party";
        assert_eq!(
            Summary::from_value(s)?.into_string(),
            "SUMMARY:Department Party\r\n"
        );
        Ok(())
    }
}
