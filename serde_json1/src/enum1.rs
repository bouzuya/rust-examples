use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Deserialize, PartialEq, Serialize)]
struct Z {
    n: i32,
}

#[derive(Clone, Debug, Eq, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
enum X {
    Y { s: String },
    Z(Z),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&X::Y {
                s: "123".to_string(),
            })?,
            r#"{"type":"Y","s":"123"}"#
        );
        assert_eq!(
            serde_json::to_string(&X::Z(Z { n: 123 }))?,
            r#"{"type":"Z","n":123}"#
        );
        Ok(())
    }
}
