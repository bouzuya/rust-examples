use crate::i_calendar::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("unique identifier")]
pub struct UniqueIdentifierError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("invalid format")]
    InvalidFormat,
    #[error("text")]
    Text(#[from] TextError),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.7>
/// uidparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UniqueIdentifier(Text);

impl From<UniqueIdentifier> for String {
    fn from(value: UniqueIdentifier) -> String {
        format!("UID:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for UniqueIdentifier {
    type Error = UniqueIdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("UID:") && value.ends_with("\r\n") {
            let text = value
                .trim_start_matches("UID:")
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
        assert_fn::<UniqueIdentifier>();

        let s = "UID:19960401T080045Z-4000F192713-0052@example.com\r\n".to_owned();
        assert_eq!(String::from(UniqueIdentifier::try_from(s.clone())?), s);
        Ok(())
    }
}
