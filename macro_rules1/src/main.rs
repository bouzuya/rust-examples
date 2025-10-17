mod error1;
mod error2;
mod macros;

macros::my_macro! {
    pub enum MyEnum {
        Variant1,
        Variant2,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PublicError(String);

pub trait MyErrorTrait {
    fn into_public_error(self) -> PublicError;
}

macros::my_macro2! {
    pub enum MyError {
        Error1(self::error1::Error1),
        Error2(self::error2::Error2),
    }
}

fn main() {
    assert_eq!(MyEnum::variants(), vec!["Variant1", "Variant2"]);
    assert_eq!(MyEnum::Variant1.to_string(), "Variant1");
    assert_eq!(MyEnum::Variant2.to_string(), "Variant2");

    assert_eq!(
        MyError::Error1(self::error1::Error1).into_public_error(),
        PublicError("error1".to_owned())
    );
    assert_eq!(
        MyError::Error2(self::error2::Error2).into_public_error(),
        PublicError("error2".to_owned())
    );

    assert_eq!(macros::my_macro3!("a"), "b");
    assert_eq!(macros::my_macro3! { "a" }, "b");
    assert_eq!(macros::my_macro3!["a"], "b");
    assert_eq!(macros::my_macro4!("a"), "b");
    assert_eq!(macros::my_macro5!("a"), "b");
}
