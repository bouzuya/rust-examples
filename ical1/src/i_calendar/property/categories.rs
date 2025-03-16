use crate::i_calendar::value_type::{Text, TextError};

#[derive(Debug, thiserror::Error)]
#[error("categories")]
pub struct CategoriesError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("text")]
    Text(#[from] TextError),
    #[error("invalid format")]
    InvalidFormat,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.2>
/// catparam not supported
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Categories(Vec<Text>);

impl Categories {
    // TODO: what is value?
    pub fn from_value(value: &str) -> Result<Self, CategoriesError> {
        Self::from_string(format!("CATEGORIES:{}\r\n", value))
    }

    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, CategoriesError> {
        if s.starts_with("CATEGORIES:") && s.ends_with("\r\n") {
            Ok(s.trim_start_matches("CATEGORIES:")
                .trim_end_matches("\r\n")
                // FIXME: Text contains `"\\,"`
                .split(',')
                .map(ToOwned::to_owned)
                .map(Text::try_from)
                .collect::<Result<Vec<Text>, TextError>>()
                .map(Self)
                .map_err(ErrorInner::Text)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        format!(
            "CATEGORIES:{}\r\n",
            self.0
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Categories>();

        let s = "CATEGORIES:APPOINTMENT,EDUCATION\r\n".to_owned();
        assert_eq!(Categories::from_string(s.clone())?.into_string(), s);

        let s = "CATEGORIES:MEETING\r\n".to_owned();
        assert_eq!(Categories::from_string(s.clone())?.into_string(), s);

        let s = "CATEGORIES:MEETING".to_owned();
        assert!(Categories::from_string(s).is_err());

        let s = "APPOINTMENT,EDUCATION";
        assert_eq!(
            Categories::from_value(s)?.into_string(),
            "CATEGORIES:APPOINTMENT,EDUCATION\r\n"
        );

        Ok(())
    }
}
