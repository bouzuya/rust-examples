mod condition;
mod expiration;
mod field;
mod value;

pub use self::condition::Condition;
pub use self::expiration::Expiration;
pub use self::field::Field;
pub use self::value::Value;

// <https://cloud.google.com/storage/docs/authentication/signatures#policy-document>
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PolicyDocument {
    pub conditions: Vec<Condition>,
    pub expiration: Expiration,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_impls<
            T: Clone
                + std::fmt::Debug
                + Eq
                + PartialEq
                + serde::Deserialize<'static>
                + serde::Serialize,
        >() {
        }
        assert_impls::<PolicyDocument>();

        let policy_document = PolicyDocument {
            conditions: vec![
                Condition::ExactMatching(Field::new("bucket")?, Value::new("travel-maps")),
                Condition::ExactMatching(Field::new("Content-Type")?, Value::new("image/jpeg")),
                Condition::StartsWith(Field::new("key")?, Value::new("")),
                Condition::ContentLengthRange(0, 1000000),
            ],
            expiration: Expiration::from_str("2020-06-16T11:11:11Z")?,
        };

        let json = r#"
{
  "conditions": [
    {"bucket": "travel-maps"},
    ["eq", "$Content-Type", "image/jpeg"],
    ["starts-with", "$key", ""],
    ["content-length-range", 0, 1000000]
  ],
  "expiration": "2020-06-16T11:11:11Z"
}
"#
        .trim();
        assert_eq!(
            serde_json::from_str::<PolicyDocument>(json)?,
            policy_document
        );
        assert_eq!(
            serde_json::to_string_pretty(&policy_document)?,
            r#"
{
  "conditions": [
    [
      "eq",
      "$bucket",
      "travel-maps"
    ],
    [
      "eq",
      "$Content-Type",
      "image/jpeg"
    ],
    [
      "starts-with",
      "$key",
      ""
    ],
    [
      "content-length-range",
      0,
      1000000
    ]
  ],
  "expiration": "2020-06-16T11:11:11Z"
}
            "#
            .trim()
        );

        Ok(())
    }
}
