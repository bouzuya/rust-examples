mod condition;
mod expiration;
mod field;
mod value;

pub use self::condition::Condition;
pub use self::expiration::Expiration;
pub use self::field::Field;
pub use self::value::Value;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
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
            T: Clone + std::fmt::Debug + Eq + PartialEq + serde::Deserialize<'static>,
        >() {
        }
        assert_impls::<PolicyDocument>();

        let json = r#"
{
  "conditions": [
    {"bucket": "travel-maps"},
    ["eq", "$Content-Type", "image/jpeg"],
    ["starts-with", "$key", ""],
    ["content-length-range", 0, 1000000]
  ],
  "expiration": "20200616T111111Z"
}
"#
        .trim();
        assert_eq!(
            serde_json::from_str::<PolicyDocument>(json)?,
            PolicyDocument {
                conditions: vec![
                    Condition::ExactMatching(Field::new("bucket")?, Value::new("travel-maps")),
                    Condition::ExactMatching(Field::new("Content-Type")?, Value::new("image/jpeg")),
                    Condition::StartsWith(Field::new("key")?, Value::new("")),
                    Condition::ContentLengthRange(0, 1000000),
                ],
                expiration: Expiration::from_str("20200616T111111Z")?,
            }
        );
        Ok(())
    }
}
