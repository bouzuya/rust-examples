fn main() {
    #[derive(derive1::VariantsFn)]
    enum E {
        #[rename]
        A,
        B,
        C,
    }
}
