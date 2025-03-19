//! Method
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.2>

use crate::i_calendar::property_parameters::OtherParam;
use crate::i_calendar::value_type::{Text, TextError};

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Method(Text, Vec<OtherParam>);

impl Method {
    pub fn new(value: Text) -> Result<Self, MethodError> {
        Ok(Self(value, Vec::new()))
    }

    pub fn with_parameters<I>(value: Text, param: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OtherParam>,
    {
        Self(
            value,
            param
                .into_iter()
                .map(Into::into)
                .collect::<Vec<OtherParam>>(),
        )
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, MethodError> {
        if s.starts_with("METHOD:") && s.ends_with("\r\n") {
            Ok(Text::from_string(
                s.trim_start_matches("METHOD:")
                    .trim_end_matches("\r\n")
                    .to_owned(),
            )
            .map(|text| Self(text, Vec::new()))
            .map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        let mut s = String::new();
        s.push_str("METHOD");
        for p in &self.1 {
            s.push(';');
            s.push_str(&p.to_escaped());
        }
        s.push(':');
        s.push_str(&self.0.to_string());
        s.push_str("\r\n");
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::i_calendar::property_parameters::{IanaParam, IanaToken, ParamValue};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<Method>();

        let s = "METHOD:REQUEST\r\n".to_owned();
        assert_eq!(Method::from_string(s.clone())?.into_string(), s);

        let s = "METHOD:REQUEST".to_owned();
        assert!(Method::from_string(s).is_err());

        let s = "REQUEST";
        assert_eq!(
            Method::new(Text::from_unescaped(s)?)?.into_string(),
            "METHOD:REQUEST\r\n"
        );

        assert_eq!(
            Method::with_parameters(
                Text::from_unescaped("REQUEST")?,
                [IanaParam::new(
                    IanaToken::from_unescaped("PARAMETER")?,
                    vec![ParamValue::from_unescaped("value")?]
                )?]
            )
            .into_string(),
            "METHOD;PARAMETER=value:REQUEST\r\n"
        );

        Ok(())
    }
}
