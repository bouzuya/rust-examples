#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum Permission {
    A,
    B,
    C,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_value(&self)
            .expect("Permission should be serializable")
            .as_str()
            .expect("Permission should serialize to a string")
            .fmt(f)
    }
}

impl std::str::FromStr for Permission {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_owned()))
    }
}

fn main() {
    for (s, p) in [
        ("A", Permission::A),
        ("B", Permission::B),
        ("C", Permission::C),
    ] {
        assert_eq!(
            <Permission as std::str::FromStr>::from_str(s).expect("should parse"),
            p
        );
        assert_eq!(p.to_string(), s);
    }
}
