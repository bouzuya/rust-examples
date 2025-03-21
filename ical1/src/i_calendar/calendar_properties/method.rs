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
}

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

    pub(in crate::i_calendar) fn to_escaped(&self) -> String {
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
    use crate::i_calendar::property_parameters::{ParamValue, XName, XParam};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<Method>();

        let s = "REQUEST";
        assert_eq!(
            Method::new(Text::from_unescaped(s)?)?.to_escaped(),
            "METHOD:REQUEST\r\n"
        );

        assert_eq!(
            Method::with_parameters(
                Text::from_unescaped("REQUEST")?,
                [XParam::new(
                    XName::from_unescaped("X-PARAM")?,
                    vec![ParamValue::from_unescaped("value")?]
                )?]
            )
            .to_escaped(),
            "METHOD;X-PARAM=value:REQUEST\r\n"
        );

        Ok(())
    }
}
