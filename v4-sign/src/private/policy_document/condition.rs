use super::{field::Field, value::Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    ExactMatching(Field, Value),
    StartsWith(Field, Value),
    ContentLengthRange(usize, usize),
}

impl<'de> serde::Deserialize<'de> for Condition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"[condition, "$field", "value"] or {"field": "value"}"#)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let (field, value) = map
                    .next_entry::<String, String>()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &"1"))?;
                let field = Field::new(field).map_err(serde::de::Error::custom)?;
                if field == Field::content_length() {
                    return Err(serde::de::Error::custom("invalid field: Content-Length"));
                }
                let value = Value::new(value);
                Ok(Condition::ExactMatching(field, value))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let condition = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &"3"))?;
                Ok(match condition {
                    "eq" | "starts-with" => {
                        let field = seq
                            .next_element::<String>()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &"3"))?;
                        let field = field.strip_prefix('$').ok_or_else(|| {
                            serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(&field),
                                &"a string starts with '$'",
                            )
                        })?;
                        let field = Field::new(field).map_err(serde::de::Error::custom)?;
                        if field == Field::content_length() {
                            return Err(serde::de::Error::custom("invalid field: Content-Length"));
                        }
                        let value = seq
                            .next_element::<String>()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &"3"))?;
                        let value = Value::new(value);
                        if condition == "eq" {
                            Condition::ExactMatching(field, value)
                        } else if condition == "starts-with" {
                            Condition::StartsWith(field, value)
                        } else {
                            unreachable!()
                        }
                    }
                    "content-length-range" => {
                        let min_range = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &"3"))?;
                        let max_range = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &"3"))?;
                        Condition::ContentLengthRange(min_range, max_range)
                    }
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "unknown condition `{}`",
                            condition
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for Condition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Condition::ExactMatching(field, value) => {
                ("eq", format!("${}", field), value.to_string()).serialize(serializer)
            }
            Condition::StartsWith(field, value) => {
                ("starts-with", format!("${}", field), value.to_string()).serialize(serializer)
            }
            Condition::ContentLengthRange(min_range, max_range) => {
                ("content-length-range", min_range, max_range).serialize(serializer)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impls() {
        fn assert_impls<
            T: Clone
                + std::fmt::Debug
                + Eq
                + PartialEq
                + serde::Deserialize<'static>
                + serde::Serialize,
        >() {
        }
        assert_impls::<Condition>();
    }

    #[test]
    fn test_eq_array() -> anyhow::Result<()> {
        let json = r#"["eq", "$Content-Type", "image/jpeg"]"#;
        let condition = serde_json::from_str::<Condition>(json)?;
        assert_eq!(
            condition,
            Condition::ExactMatching(Field::new("Content-Type")?, Value::new("image/jpeg"))
        );

        let json = r#"["eq", "$Content-Length", "123"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid field: Content-Length at line 1 column 26"
        );

        let json = r#"["eq"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid length 1, expected 3 at line 1 column 6"
        );

        let json = r#"["eq", "$Content-Type"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid length 2, expected 3 at line 1 column 23"
        );

        let json = r#"["eq", "$Content-Type", "image/jpeg", "foo"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "trailing characters at line 1 column 39"
        );
        Ok(())
    }

    #[test]
    fn test_eq_object() -> anyhow::Result<()> {
        let json = r#"{"Content-Type": "image/jpeg"}"#;
        let condition = serde_json::from_str::<Condition>(json)?;
        assert_eq!(
            condition,
            Condition::ExactMatching(Field::new("Content-Type")?, Value::new("image/jpeg"))
        );

        let json = r#"{"Content-Length": "123"}"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid field: Content-Length at line 1 column 25"
        );

        let json = r#"{"Content-Type": "image/jpeg", "key": "foo"}"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "trailing comma at line 1 column 30"
        );
        Ok(())
    }

    #[test]
    fn test_starts_with() -> anyhow::Result<()> {
        let json = r#"["starts-with", "$key", ""]"#;
        let condition = serde_json::from_str::<Condition>(json)?;
        assert_eq!(
            condition,
            Condition::StartsWith(Field::new("key")?, Value::new(""))
        );

        let json = r#"["starts-with", "$Content-Length", ""]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid field: Content-Length at line 1 column 35"
        );

        let json = r#"["starts-with", "$", ""]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "unknown field:  at line 1 column 21"
        );

        let json = r#"["starts-with", "key", ""]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"invalid value: string "key", expected a string starts with '$' at line 1 column 23"#
        );

        let json = r#"["starts-with"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid length 1, expected 3 at line 1 column 15"
        );

        let json = r#"["starts-with", "$key"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid length 2, expected 3 at line 1 column 23"
        );

        let json = r#"["starts-with", "$key", 123]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid type: integer `123`, expected a string at line 1 column 27"
        );

        let json = r#"["starts-with", "$key", "", "foo"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "trailing characters at line 1 column 29"
        );
        Ok(())
    }

    #[test]
    fn test_content_length_range() -> anyhow::Result<()> {
        let json = r#"["content-length-range", 0, 1000000]"#;
        let condition = serde_json::from_str::<Condition>(json)?;
        assert_eq!(condition, Condition::ContentLengthRange(0, 1000000));

        let json = r#"["content-length-range"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid length 1, expected 3 at line 1 column 24"
        );

        let json = r#"["content-length-range", 0]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid length 2, expected 3 at line 1 column 27"
        );

        let json = r#"["content-length-range", 0, "1000000"]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"invalid type: string "1000000", expected usize at line 1 column 37"#
        );

        let json = r#"["content-length-range", 0, 1000000, 123]"#;
        let result = serde_json::from_str::<Condition>(json);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "trailing characters at line 1 column 38"
        );
        Ok(())
    }

    #[test]
    fn test_serialize() -> anyhow::Result<()> {
        for (condition, json) in [
            (
                Condition::ExactMatching(Field::new("Content-Type")?, Value::new("image/jpeg")),
                r#"["eq","$Content-Type","image/jpeg"]"#,
            ),
            (
                Condition::StartsWith(Field::new("key")?, Value::new("")),
                r#"["starts-with","$key",""]"#,
            ),
            (
                Condition::ContentLengthRange(0, 1000000),
                r#"["content-length-range",0,1000000]"#,
            ),
        ] {
            assert_eq!(serde_json::to_string(&condition)?, json);
            assert_eq!(serde_json::from_str::<Condition>(json)?, condition);
        }
        Ok(())
    }
}
