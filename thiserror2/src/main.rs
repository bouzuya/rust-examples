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

    println!("Hello, world!");
}
