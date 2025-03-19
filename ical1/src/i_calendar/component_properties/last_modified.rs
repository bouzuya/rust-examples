//! Last Modified
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.3>

use crate::i_calendar::{property_parameters::OtherParam, value_type::DateTime};

#[derive(Debug, thiserror::Error)]
#[error("last modified")]
pub struct LastModifiedError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("UTC time format required")]
    UtcTimeFormatRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LastModified(DateTime, Vec<OtherParam>);

impl LastModified {
    pub fn new(value: DateTime) -> Result<Self, LastModifiedError> {
        Self::with_parameters(value, Vec::<OtherParam>::new())
    }

    pub fn with_parameters<I>(value: DateTime, param: I) -> Result<Self, LastModifiedError>
    where
        I: IntoIterator,
        I::Item: Into<OtherParam>,
    {
        if !value.to_escaped().ends_with('Z') {
            return Err(ErrorInner::UtcTimeFormatRequired)?;
        }
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
        s.push_str("LAST-MODIFIED");
        for p in &self.1 {
            s.push(';');
            s.push_str(&p.to_escaped());
        }
        s.push(':');
        s.push_str(&self.0.to_escaped());
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
        assert_fn::<LastModified>();

        assert_eq!(
            LastModified::new(DateTime::try_from("19960817T133000Z".to_owned())?)?.to_escaped(),
            "LAST-MODIFIED:19960817T133000Z\r\n"
        );

        assert_eq!(
            LastModified::with_parameters(
                DateTime::try_from("19960817T133000Z".to_owned())?,
                [IanaParam::new(
                    IanaToken::from_unescaped("IANA-TOKEN")?,
                    vec![ParamValue::from_unescaped("param-value")?]
                )?]
            )?
            .to_escaped(),
            "LAST-MODIFIED;IANA-TOKEN=param-value:19960817T133000Z\r\n"
        );

        Ok(())
    }
}
