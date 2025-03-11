use crate::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("classification")]
pub struct ClassificationError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
    #[error("invalid format")]
    InvalidFormat,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3>
/// classparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Classification(Text);

impl From<Classification> for String {
    fn from(value: Classification) -> String {
        format!("CLASS:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for Classification {
    type Error = ClassificationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("CLASS:") && value.ends_with("\r\n") {
            let text = value
                .trim_start_matches("CLASS:")
                .trim_end_matches("\r\n")
                .to_owned();
            Ok(Text::try_from(text).map(Self).map_err(ErrorInner::Text)?)
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
        assert_fn::<Classification>();

        let s = "CLASS:PUBLIC\r\n".to_owned();
        assert_eq!(String::from(Classification::try_from(s.clone())?), s);

        let s = "CLASS:PUBLIC".to_owned();
        assert!(Classification::try_from(s).is_err());
        Ok(())
    }
}
