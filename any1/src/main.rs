mod async_graphql_like;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_type_id() {
        trait MyTrait {}

        impl MyTrait for i32 {}
        impl MyTrait for u8 {}

        assert_ne!(
            std::any::Any::type_id(&123_i32),
            std::any::Any::type_id("abc")
        );
        assert_eq!(
            std::any::Any::type_id(&123_i32),
            std::any::Any::type_id(&456_i32)
        );
        assert_ne!(
            std::any::Any::type_id(&1_i32),
            std::any::Any::type_id(&1_u8)
        );

        let a = Box::new(1_i32);
        let b = Box::new(1_u8);
        assert_ne!(std::any::Any::type_id(&a), std::any::Any::type_id(&b));

        let x: Box<dyn MyTrait> = a;
        let y: Box<dyn MyTrait> = b;
        assert_eq!(std::any::Any::type_id(&x), std::any::Any::type_id(&y));
    }

    #[test]
    fn test_type_name() {
        trait MyTrait {}

        assert_eq!(std::any::type_name::<i32>(), "i32");
        assert_eq!(std::any::type_name::<&i32>(), "&i32");
        assert_eq!(std::any::type_name::<&'static i32>(), "&i32");
        assert_eq!(std::any::type_name::<&mut i32>(), "&mut i32");
        assert_eq!(std::any::type_name::<&str>(), "&str");
        assert_eq!(std::any::type_name::<Box<i32>>(), "alloc::boxed::Box<i32>");
        assert_eq!(
            std::any::type_name::<Box<&i32>>(),
            "alloc::boxed::Box<&i32>"
        );
        assert_eq!(
            std::any::type_name::<Box<&str>>(),
            "alloc::boxed::Box<&str>"
        );
        assert_eq!(
            std::any::type_name::<Box<dyn MyTrait>>(),
            "alloc::boxed::Box<dyn any1::tests::test_type_name::MyTrait>"
        );
    }
}
