//! Date-Time Created
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.1>

use crate::i_calendar::{
    property_parameters::OtherParam,
    value_type::{DateTime, DateTimeError},
};

#[derive(Debug, thiserror::Error)]
#[error("date-time created")]
pub struct DateTimeCreatedError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("date-time")]
    DateTime(#[from] DateTimeError),
    #[error("UTC time format required")]
    UtcTimeFormatRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeCreated(DateTime, Vec<OtherParam>);

impl DateTimeCreated {
    pub fn new(value: DateTime) -> Result<Self, DateTimeCreatedError> {
        Self::with_parameters(value, Vec::<OtherParam>::new())
    }

    pub fn with_parameters<I>(value: DateTime, param: I) -> Result<Self, DateTimeCreatedError>
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
        s.push_str("CREATED");
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
    use crate::i_calendar::property_parameters::{ParamValue, XName, XParam};

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<DateTimeCreated>();

        assert_eq!(
            DateTimeCreated::new(DateTime::try_from("19960401T150000Z".to_owned())?)?.to_escaped(),
            "CREATED:19960401T150000Z\r\n"
        );

        assert_eq!(
            DateTimeCreated::with_parameters(
                DateTime::try_from("19960401T150000Z".to_owned())?,
                vec![XParam::new(
                    XName::from_unescaped("X-PARAM")?,
                    vec![ParamValue::from_unescaped("value")?]
                )?],
            )?
            .to_escaped(),
            "CREATED;X-PARAM=value:19960401T150000Z\r\n"
        );

        Ok(())
    }
}
