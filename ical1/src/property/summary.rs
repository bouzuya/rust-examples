use crate::value_type::{Text, TextError};

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

impl From<Summary> for String {
    fn from(value: Summary) -> String {
        format!("SUMMARY:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for Summary {
    type Error = SummaryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("SUMMARY:") && value.ends_with("\r\n") {
            let date_time = value
                .trim_start_matches("SUMMARY:")
                .trim_end_matches("\r\n")
                .to_owned();
            Ok(Text::try_from(date_time)
                .map(Summary)
                .map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
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
        assert_eq!(String::from(Summary::try_from(s.clone())?), s);

        let s = "SUMMARY:Department Party".to_owned();
        assert!(Summary::try_from(s).is_err());
        Ok(())
    }
}
