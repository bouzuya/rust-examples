use std::error::Error;

fn example1() {
    #[derive(Debug, thiserror::Error)]
    #[error("unit struct error")]
    struct UnitStructError;

    fn assert_impls<T: std::fmt::Debug + std::fmt::Display + std::error::Error>() {}
    assert_impls::<UnitStructError>();
    assert_eq!(UnitStructError.to_string(), "unit struct error");
}

fn example2() {
    #[derive(Debug, thiserror::Error)]
    #[error("tuple struct error: {0}")]
    struct TupleStructError(#[from] std::io::Error);

    fn assert_impls<T: std::fmt::Debug + std::fmt::Display + std::error::Error>() {}
    assert_impls::<TupleStructError>();
    assert_eq!(
        format!("{}", TupleStructError(std::io::Error::other("error1"))),
        "tuple struct error: error1"
    );
    assert_eq!(
        format!(
            "{}",
            // `#[from]` は `impl From<std::io::Error> for TupleStructError` を提供する
            TupleStructError::from(std::io::Error::other("error2"))
        ),
        "tuple struct error: error2"
    );
    assert_eq!(
        format!(
            "{:?}",
            // `#[from]` のフィールドは `std::error::Error` trait の `source()` にも使用される
            TupleStructError(std::io::Error::other("error3")).source()
        ),
        "Some(Custom { kind: Other, error: \"error3\" })"
    );
}

fn example3() {
    #[derive(Debug, thiserror::Error)]
    #[error("struct error: {cause:?}")]
    struct StructError {
        #[source]
        cause: std::io::Error,
        // `#[source]` の代わりに↓のようなフィールド名 `source` でも良い
        // source: std::io::Error,
    }

    fn assert_impls<T: std::fmt::Debug + std::fmt::Display + std::error::Error>() {}
    assert_impls::<StructError>();
    assert_eq!(
        format!(
            "{}",
            StructError {
                cause: std::io::Error::other("error1")
            }
        ),
        "struct error: Custom { kind: Other, error: \"error1\" }"
    );
    // `#[source]` は `From` trait を実装しないので、↓はコンパイルエラーになる
    // StructError::from(std::io::Error::other("error2"))
    assert_eq!(
        format!(
            "{:?}",
            // `#[source]` のフィールドは `std::error::Error` trait の `source()` にも使用される
            StructError {
                cause: std::io::Error::other("error2")
            }
            .source()
        ),
        "Some(Custom { kind: Other, error: \"error2\" })"
    );
}

fn example4() {
    #[derive(Debug, thiserror::Error)]
    enum EnumError {
        #[error("io error")]
        Io(#[source] std::io::Error),
        #[error(transparent)]
        From(#[from] Box<dyn std::error::Error + Send + Sync>),
        #[error(transparent)]
        Unknown(Box<dyn std::error::Error + Send + Sync>),
    }

    fn assert_impls<T: std::fmt::Debug + std::fmt::Display + std::error::Error>() {}
    assert_impls::<EnumError>();
    assert_eq!(
        format!("{}", EnumError::Io(std::io::Error::other("error1"))),
        "io error"
    );
    assert_eq!(
        format!(
            "{}",
            EnumError::from(Box::<dyn std::error::Error + Send + Sync>::from("error2"))
        ),
        "error2"
    );
    assert_eq!(
        format!(
            "{:?}",
            EnumError::from(Box::<dyn std::error::Error + Send + Sync>::from("error2")).source()
        ),
        "None"
    );
    assert_eq!(format!("{}", EnumError::Unknown("error3".into())), "error3");
}

fn main() {
    example1();
    example2();
    example3();
    example4();
}
