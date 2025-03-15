//! Method
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.2>

use crate::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("method")]
pub struct MethodError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
    #[error("invalid format")]
    InvalidFormat,
}

/// metparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Method(Text);

impl From<Method> for String {
    fn from(value: Method) -> String {
        format!("METHOD:{}\r\n", String::from(value.0))
    }
}

impl TryFrom<String> for Method {
    type Error = MethodError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("METHOD:") && value.ends_with("\r\n") {
            Ok(Text::try_from(
                value
                    .trim_start_matches("METHOD:")
                    .trim_end_matches("\r\n")
                    .to_owned(),
            )
            .map(Self)
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
        assert_fn::<Method>();

        let s = "METHOD:REQUEST\r\n".to_owned();
        assert_eq!(String::from(Method::try_from(s.clone())?), s);

        let s = "METHOD:REQUEST".to_owned();
        assert!(Method::try_from(s).is_err());
        Ok(())
    }
}
