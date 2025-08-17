fn main() {
    #[derive(derive1::VariantsFn)]
    enum E {
        A,
        B,
        C,
    }
    assert_eq!(E::variants(), &["A", "B", "C"]);
}
