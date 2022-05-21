// <https://stackoverflow.com/q/44301748>
// <https://github.com/serde-rs/serde/issues/1344>

#[cfg(test)]
mod tests {
    use serde::{
        de::{self, Unexpected},
        Deserialize, Deserializer,
    };

    fn bool_from_01<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            s => Err(de::Error::invalid_value(
                Unexpected::Str(s),
                &r#""0" or "1""#,
            )),
        }
    }

    fn option_bool_from_01<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            Some(s) => match s.as_str() {
                "0" => Ok(Some(false)),
                "1" => Ok(Some(true)),
                s => Err(de::Error::invalid_value(
                    Unexpected::Str(s),
                    &r#""0" or "1""#,
                )),
            },
            None => Ok(None),
        }
    }

    #[test]
    fn x_status() -> anyhow::Result<()> {
        #[derive(Deserialize)]
        struct X {
            #[serde(deserialize_with = "bool_from_01")]
            is1: bool,
            #[serde(deserialize_with = "bool_from_01")]
            is2: bool,
            #[serde(default)]
            #[serde(deserialize_with = "option_bool_from_01")]
            is3: Option<bool>,
            #[serde(default)]
            #[serde(deserialize_with = "option_bool_from_01")]
            is4: Option<bool>,
            #[serde(default)]
            #[serde(deserialize_with = "option_bool_from_01")]
            is5: Option<bool>,
        }
        let x: X = serde_json::from_str(r#"{"is1":"0","is2":"1","is4":"0","is5":"1"}"#)?;
        assert!(!x.is1);
        assert!(x.is2);
        assert!(x.is3.is_none());
        assert_eq!(x.is4, Some(false));
        assert_eq!(x.is5, Some(true));
        Ok(())
    }
}
