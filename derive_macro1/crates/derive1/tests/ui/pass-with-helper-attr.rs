fn main() {
    #[derive(derive1::VariantsFn)]
    enum E {
        #[rename = "X"]
        A,
        B,
        C,
    }
    assert_eq!(E::variants(), &["X", "B", "C"]);
}
