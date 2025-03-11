use crate::value_type::{Text, TextError};

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

impl From<Categories> for String {
    fn from(value: Categories) -> String {
        format!(
            "CATEGORIES:{}\r\n",
            value
                .0
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl TryFrom<String> for Categories {
    type Error = CategoriesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("CATEGORIES:") && value.ends_with("\r\n") {
            Ok(value
                .trim_start_matches("CATEGORIES:")
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Categories>();

        let s = "CATEGORIES:APPOINTMENT,EDUCATION\r\n".to_owned();
        assert_eq!(String::from(Categories::try_from(s.clone())?), s);

        let s = "CATEGORIES:MEETING\r\n".to_owned();
        assert_eq!(String::from(Categories::try_from(s.clone())?), s);

        let s = "CATEGORIES:MEETING".to_owned();
        assert!(Categories::try_from(s).is_err());
        Ok(())
    }
}
