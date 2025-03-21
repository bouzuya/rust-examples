//! Calendar Scale
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.1>

use crate::i_calendar::{
    property_parameters::OtherParam,
    value_type::{Text, TextError},
};

#[derive(Debug, thiserror::Error)]
#[error("calendar scale")]
pub struct CalendarScaleError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("invalid calvalue")]
    InvalidCalvalue,
    #[error("text")]
    Text(#[from] TextError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarScale(Vec<OtherParam>);

impl CalendarScale {
    pub fn new(value: Text) -> Result<Self, CalendarScaleError> {
        Self::with_parameters(value, Vec::<OtherParam>::new())
    }

    pub fn with_parameters<I>(value: Text, param: I) -> Result<Self, CalendarScaleError>
    where
        I: IntoIterator,
        I::Item: Into<OtherParam>,
    {
        if value.to_string() != "GREGORIAN" {
            return Err(ErrorInner::InvalidCalvalue)?;
        }
        Ok(Self(
            param
                .into_iter()
                .map(Into::into)
                .collect::<Vec<OtherParam>>(),
        ))
    }

    pub(in crate::i_calendar) fn to_escaped(&self) -> String {
        let mut s = String::new();
        s.push_str("CALSCALE");
        for p in &self.0 {
            s.push(';');
            s.push_str(&p.to_escaped());
        }
        s.push_str(":GREGORIAN\r\n");
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
        assert_fn::<CalendarScale>();

        let s = "GREGORIAN";
        assert_eq!(
            CalendarScale::new(Text::from_unescaped(s)?)?.to_escaped(),
            "CALSCALE:GREGORIAN\r\n"
        );

        assert_eq!(
            CalendarScale::with_parameters(
                Text::from_unescaped("GREGORIAN")?,
                [IanaParam::new(
                    IanaToken::from_unescaped("IANA-TOKEN")?,
                    vec![ParamValue::from_unescaped("param-value")?]
                )?]
            )?
            .to_escaped(),
            "CALSCALE;IANA-TOKEN=param-value:GREGORIAN\r\n"
        );

        Ok(())
    }
}
