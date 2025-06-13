#[derive(Debug, thiserror::Error)]
#[error("... {type} ...")]
struct Error1 {
    r#type: String,
}

// #[error("... {0} {} ...")]
// => compile error
#[derive(Debug, thiserror::Error)]
#[error("... {0} {1} ...")]
struct Error2(i32, String);

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct Error3(Box<dyn std::error::Error + Send + Sync>);

#[derive(Debug, thiserror::Error)]
#[error("error4")]
struct Error4(#[source] Box<dyn std::error::Error + Send + Sync>);

fn main() {
    assert_eq!(
        format!(
            "{}",
            Error1 {
                r#type: "abc".to_owned()
            }
        ),
        "... abc ..."
    );

    let e3 = Error3(Box::new(std::io::Error::other("inner")));
    assert_eq!(e3.to_string(), "inner");
    assert_eq!(
        format!("{:?}", e3),
        "Error3(Custom { kind: Other, error: \"inner\" })"
    );
    assert_eq!(format!("{:?}", anyhow::anyhow!(e3)), "inner");

    let e4 = Error4(Box::new(std::io::Error::other("inner")));
    assert_eq!(e4.to_string(), "error4");
    assert_eq!(
        format!("{:?}", e4),
        "Error4(Custom { kind: Other, error: \"inner\" })"
    );
    assert_eq!(
        format!("{:?}", anyhow::anyhow!(e4)),
        ["error4", "", "Caused by:", "    inner"].join("\n")
    );
}
