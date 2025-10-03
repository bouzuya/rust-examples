fn main() {
    #[derive(serde::Deserialize)]
    struct Package {
        name: String,
        dependencies: std::collections::BTreeMap<String, Dependency>,
    }

    #[derive(serde::Deserialize)]
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
}
