#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value(String);

impl Value {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn assert_impls<T: Clone + std::fmt::Debug + Eq + PartialEq>() {}
        assert_impls::<Value>();

        let value = Value::new("value");
        assert_eq!(value, Value("value".to_string()));

        let value = Value::new("");
        assert_eq!(value, Value("".to_string()));
    }
}
