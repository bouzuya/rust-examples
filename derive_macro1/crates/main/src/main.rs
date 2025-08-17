#[derive(derive1::VariantsFn)]
enum E1 {
    #[rename = "X"]
    A,
    B(i32),
    C {
        b: bool,
    },
}

#[derive(derive1::VariantsFn)]
enum E2 {}

fn main() {
    assert_eq!(E1::variants(), &["A", "B", "C"]);
    assert_eq!(E2::variants(), &[] as &'static [&'static str]);
}
