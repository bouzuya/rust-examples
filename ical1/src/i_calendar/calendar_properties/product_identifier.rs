//! Product Identifier
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.3>

use crate::i_calendar::{
    property_parameters::OtherParam,
    value_type::{Text, TextError},
};

#[derive(Debug, thiserror::Error)]
#[error("unique identifier")]
pub struct ProductIdentifierError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductIdentifier(Text, Vec<OtherParam>);

impl ProductIdentifier {
    pub fn new(value: Text) -> Result<Self, ProductIdentifierError> {
        Self::with_parameters(value, Vec::<OtherParam>::new())
    }

    pub fn with_parameters<I>(value: Text, param: I) -> Result<Self, ProductIdentifierError>
    where
        I: IntoIterator,
        I::Item: Into<OtherParam>,
    {
        Ok(Self(
            value,
            param
                .into_iter()
                .map(Into::into)
                .collect::<Vec<OtherParam>>(),
        ))
    }

    pub(in crate::i_calendar) fn to_escaped(&self) -> String {
        let mut s = String::new();
        s.push_str("PRODID");
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
        assert_fn::<ProductIdentifier>();

        let s = "-//ABC Corporation//NONSGML My Product//EN";
        assert_eq!(
            ProductIdentifier::new(Text::from_unescaped(s)?)?.to_escaped(),
            "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n"
        );

        assert_eq!(
            ProductIdentifier::with_parameters(
                Text::from_unescaped("-//ABC Corporation//NONSGML My Product//EN")?,
                vec![XParam::new(
                    XName::from_unescaped("X-PARAM")?,
                    vec![ParamValue::from_unescaped("value")?]
                )?]
            )?
            .to_escaped(),
            "PRODID;X-PARAM=value:-//ABC Corporation//NONSGML My Product//EN\r\n"
        );

        Ok(())
    }
}
