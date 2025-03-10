#[derive(Debug, thiserror::Error)]
#[error("text")]
pub struct TextError {
    // TODO: improve error message
    _private: (),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Text(String);

impl From<Text> for String {
    fn from(value: Text) -> Self {
        // FIXME: escape sequence
        value.0
    }
}

impl TryFrom<String> for Text {
    type Error = TextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // FIXME: unescape sequence
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Text>();

        let s = "Project XYZ Final Review\\nConference Room - 3B\\nCome Prepared.".to_owned();
        assert_eq!(String::from(Text::try_from(s.clone())?), s);
        Ok(())
    }
}
