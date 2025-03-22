//! Description
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.5>

use crate::i_calendar::{Text, value_type::DateTimeError};

#[derive(Debug, thiserror::Error)]
#[error("description")]
pub struct DescriptionError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("date-time")]
    DateTime(#[from] DateTimeError),
}

/// altrepparam, languageparam, other-param is not implemented
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Description(Text);

impl Description {
    pub fn new(value: Text) -> Result<Self, DescriptionError> {
        Ok(Self(value))
    }

    pub(in crate::i_calendar) fn to_escaped(&self) -> String {
        let mut s = String::new();
        s.push_str("DESCRIPTION:");
        s.push_str(&self.0.clone().into_string());
        s.push_str("\r\n");
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<Description>();

        assert_eq!(
            Description::new(Text::from_string(
                "Meeting to provide technical review for \"Phoenix\" design.\nHappy Face Conference Room. Phoenix design team MUST attend this meeting.\nRSVP to team leader."
                    .to_owned()
            )?)?
            .to_escaped(),
            "DESCRIPTION:Meeting to provide technical review for \"Phoenix\" design.\nHappy Face Conference Room. Phoenix design team MUST attend this meeting.\nRSVP to team leader.\r\n"
        );

        Ok(())
    }
}
