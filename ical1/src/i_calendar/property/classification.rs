use crate::i_calendar::value_type::{Text, TextError};

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

impl Classification {
    // TODO: what is value?
    pub fn from_value(value: &str) -> Result<Self, ClassificationError> {
        Self::from_string(format!("CLASS:{}\r\n", value))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, ClassificationError> {
        if s.starts_with("CLASS:") && s.ends_with("\r\n") {
            let text = s
                .trim_start_matches("CLASS:")
                .trim_end_matches("\r\n")
                .to_owned();
            Ok(Text::try_from(text).map(Self).map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!("CLASS:{}\r\n", String::from(self.0))
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
        assert_eq!(Classification::from_string(s.clone())?.into_string(), s);

        let s = "CLASS:PUBLIC".to_owned();
        assert!(Classification::from_string(s).is_err());

        let s = "PUBLIC";
        assert_eq!(
            Classification::from_value(s)?.into_string(),
            "CLASS:PUBLIC\r\n"
        );

        Ok(())
    }
}
