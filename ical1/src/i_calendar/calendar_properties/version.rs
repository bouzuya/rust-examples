//! Version
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.4>

use crate::i_calendar::{
    property_parameters::OtherParam,
    value_type::{Text, TextError},
};

#[derive(Debug, thiserror::Error)]
#[error("version")]
pub struct VersionError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version(Text, Vec<OtherParam>);

impl Version {
    pub fn new(value: Text) -> Result<Self, VersionError> {
        Self::with_parameters(value, Vec::<OtherParam>::new())
    }

    pub fn with_parameters<I>(value: Text, param: I) -> Result<Self, VersionError>
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
        s.push_str("VERSION");
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
        assert_fn::<Version>();

        assert_eq!(
            Version::new(Text::from_unescaped("2.0")?)?.to_escaped(),
            "VERSION:2.0\r\n"
        );

        assert_eq!(
            Version::with_parameters(
                Text::from_unescaped("2.0")?,
                vec![XParam::new(
                    XName::from_unescaped("X-PARAM")?,
                    vec![ParamValue::from_unescaped("value")?]
                )?],
            )?
            .to_escaped(),
            "VERSION;X-PARAM=value:2.0\r\n"
        );

        Ok(())
    }
}
