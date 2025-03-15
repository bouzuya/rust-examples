use crate::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("unique identifier")]
pub struct ProductIdentifierError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("invalid format")]
    InvalidFormat,
    #[error("text")]
    Text(#[from] TextError),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.7>
/// pidparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProductIdentifier(Text);

impl From<ProductIdentifier> for String {
    fn from(value: ProductIdentifier) -> String {
        format!("PRODID:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for ProductIdentifier {
    type Error = ProductIdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("PRODID:") && value.ends_with("\r\n") {
            let text = value
                .trim_start_matches("PRODID:")
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
        assert_fn::<ProductIdentifier>();

        let s = "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n".to_owned();
        assert_eq!(String::from(ProductIdentifier::try_from(s.clone())?), s);

        let s = "PRODID:-//ABC Corporation//NONSGML My Product//EN".to_owned();
        assert!(ProductIdentifier::try_from(s.clone()).is_err());
        Ok(())
    }
}
