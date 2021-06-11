fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn option_test() {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct T {
            k: Option<String>,
        }

        assert_eq!(
            serde_json::from_str::<T>(r#"{"k":"k1"}"#).unwrap(),
            T {
                k: Some("k1".to_string()),
            }
        );

        assert_eq!(
            serde_json::from_str::<T>(r#"{"k":null}"#).unwrap(),
            T { k: None }
        );

        assert_eq!(serde_json::from_str::<T>(r#"{}"#).unwrap(), T { k: None });
    }

    #[test]
    fn default_value_test() {
        #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct T {
            a: Vec<i64>,
        }

        assert_eq!(
            serde_json::from_str::<T>(r#"{"a":[1]}"#).unwrap(),
            T { a: vec![1] }
        );

        assert_eq!(serde_json::from_str::<T>(r#"{"a":null}"#).is_err(), true);
        assert_eq!(serde_json::from_str::<T>(r#"{}"#).is_err(), true);

        #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
        struct U {
            #[serde(default)]
            a: Vec<i64>,
        }
        assert_eq!(
            serde_json::from_str::<U>(r#"{"a":[1]}"#).unwrap(),
            U { a: vec![1] }
        );

        assert_eq!(serde_json::from_str::<U>(r#"{"a":null}"#).is_err(), true);
        assert_eq!(
            serde_json::from_str::<U>(r#"{}"#).unwrap(),
            U {
                a: Vec::<i64>::default()
            }
        );
    }
}
