fn main() {
    #[derive(Debug, serde::Deserialize)]
    struct Package {
        name: String,
        dependencies: std::collections::BTreeMap<String, Dependency>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Dependency {
        version: String,
    }

    // dependencies.serde.version is a number, not a string
    let s = r#"{
        "name": "demo",
        "dependencies": {
            "serde": {
                "version": 1
            }
        }
    }"#;
    let deserializer = &mut serde_json::Deserializer::from_str(s);
    let result: Result<Package, _> = serde_path_to_error::deserialize(deserializer);
    match result {
        Ok(_) => panic!("expected a type error"),
        Err(err) => {
            let path = err.path().to_string();
            assert_eq!(path, "dependencies.serde.version");
        }
    }

    let value = serde_firestore_value::google::firestore::v1::Value {
        value_type: Some(
            serde_firestore_value::google::firestore::v1::value::ValueType::MapValue(
                serde_firestore_value::google::firestore::v1::MapValue {
                    fields: std::collections::HashMap::from_iter([
                        ("name".to_owned(), serde_firestore_value::google::firestore::v1::Value {
                            value_type: Some(
                                serde_firestore_value::google::firestore::v1::value::ValueType::StringValue(
                                    "demo2".to_string(),
                                ),
                            ),
                        }),
                        ("dependencies".to_owned(), serde_firestore_value::google::firestore::v1::Value {
                            value_type: Some(
                                serde_firestore_value::google::firestore::v1::value::ValueType::MapValue(
                                    serde_firestore_value::google::firestore::v1::MapValue {
                                        fields: std::collections::HashMap::from_iter([
                                            ("serde".to_owned(), serde_firestore_value::google::firestore::v1::Value {
                                                value_type: Some(
                                                    serde_firestore_value::google::firestore::v1::value::ValueType::MapValue(
                                                        serde_firestore_value::google::firestore::v1::MapValue {
                                                            fields: std::collections::HashMap::from_iter([
                                                                ("version".to_owned(), serde_firestore_value::google::firestore::v1::Value {
                                                                    value_type: Some(
                                                                        // This should be a StringValue, but we use an IntegerValue to trigger a type error
                                                                        serde_firestore_value::google::firestore::v1::value::ValueType::IntegerValue(1),
                                                                    ),
                                                                }),
                                                            ]),
                                                        },
                                                    ),
                                                ),
                                            }),
                                        ]),
                                    },
                                ),
                            ),
                        }),
                    ])
                }
            ),
        ),
    };

    let deserializer = serde_firestore_value::Deserializer::new(&value);
    let result: Result<Package, _> = serde::de::Deserialize::deserialize(deserializer);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid type: integer value, expected string value"
    );

    let deserializer = serde_firestore_value::Deserializer::new(&value);
    let result: Result<Package, _> = serde_path_to_error::deserialize(deserializer);
    match result {
        Ok(_) => panic!("expected a type error"),
        Err(err) => {
            let path = err.path().to_string();
            assert_eq!(path, "dependencies.serde.version");
            assert_eq!(
                err.to_string(),
                "dependencies.serde.version: invalid type: integer value, expected string value"
            );
        }
    }

    let result: Result<Package, _> = serde_firestore_value::from_value(&value);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid type: integer value, expected string value"
    );

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct Package2 {
        name: String,
        dependencies: std::collections::BTreeMap<String, Dependency2>,
    }

    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct Dependency2 {
        version: i32,
    }

    let result: Result<Package2, _> = serde_firestore_value::from_value(&value);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        Package2 {
            name: "demo2".to_string(),
            dependencies: std::collections::BTreeMap::from_iter([(
                "serde".to_owned(),
                Dependency2 { version: 1 }
            ),]),
        }
    );
}
