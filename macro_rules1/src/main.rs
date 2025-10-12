mod macros;

macros::my_macro! {
    pub enum MyEnum {
        Variant1,
        Variant2,
    }
}

fn main() {
    assert_eq!(MyEnum::variants(), vec!["Variant1", "Variant2"]);
    assert_eq!(MyEnum::Variant1.to_string(), "Variant1");
    assert_eq!(MyEnum::Variant2.to_string(), "Variant2");
}
