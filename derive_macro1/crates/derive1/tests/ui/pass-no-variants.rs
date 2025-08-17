fn main() {
    #[derive(derive1::VariantsFn)]
    enum E {}
    assert_eq!(E::variants(), &[] as &[&str]);
}
