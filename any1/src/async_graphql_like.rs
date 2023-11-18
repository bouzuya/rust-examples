#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    struct Request(HashMap<std::any::TypeId, Box<dyn std::any::Any + Sync + Send>>);

    impl Request {
        fn data(&mut self, data: impl std::any::Any + Sync + Send) {
            self.0.insert(data.type_id(), Box::new(data));
        }
    }

    struct Context<'a>(&'a Request);

    impl<'a> Context<'a> {
        fn data<D: std::any::Any + Send + Sync>(&self) -> Option<&'a D> {
            let request = self.0;
            request
                .0
                .get(&std::any::TypeId::of::<D>())
                .and_then(|d| d.downcast_ref::<D>())
        }
    }

    #[test]
    fn test_basic_types() {
        let mut request = Request(HashMap::new());
        request.data(123_i32);
        request.data("abc".to_string());
        let context = Context(&request);

        assert_eq!(context.data::<u8>(), None);
        assert_eq!(context.data::<i32>(), Some(&123));
        assert_eq!(context.data::<String>(), Some(&"abc".to_string()));
    }

    #[test]
    fn test_struct() {
        #[derive(Debug, Eq, PartialEq)]
        struct MyStruct {
            i: i32,
            s: String,
        }

        let mut request = Request(HashMap::new());
        request.data(MyStruct {
            i: 123_i32,
            s: "abc".to_string(),
        });
        let context = Context(&request);

        assert_eq!(context.data::<u8>(), None);
        assert_eq!(
            context.data::<MyStruct>(),
            Some(&MyStruct {
                i: 123_i32,
                s: "abc".to_string(),
            })
        );
    }

    #[test]
    #[allow(unused_variables)]
    fn test_trait() {
        trait MyTrait {
            fn f(&self) -> i32;
        }

        #[derive(Debug, Eq, PartialEq)]
        struct MyStruct {
            i: i32,
        }

        impl MyTrait for MyStruct {
            fn f(&self) -> i32 {
                self.i
            }
        }

        let mut request = Request(HashMap::new());
        request.data(MyStruct { i: 123_i32 });
        let context = Context(&request);

        // add `dyn` keyword before this trait: `dyn `
        // context.data::<MyTrait>()

        // the size for values of type `dyn tests::test_trait::MyTrait` cannot be known at compilation time
        // the trait `std::marker::Sized` is not implemented for `dyn tests::test_trait::MyTrait`
        // context.data::<dyn MyTrait>();
    }

    #[test]
    fn test_boxed_trait() {
        trait MyTrait {
            fn f(&self) -> i32;
        }

        #[derive(Debug, Eq, PartialEq)]
        struct MyStruct {
            i: i32,
        }

        impl MyTrait for MyStruct {
            fn f(&self) -> i32 {
                self.i
            }
        }

        let mut request = Request(HashMap::new());
        request.data(Box::new(MyStruct { i: 123_i32 }));
        let context = Context(&request);

        // `dyn tests::test_trait2::MyTrait` cannot be sent between threads safely
        // the trait `std::marker::Send` is not implemented for `dyn tests::test_trait2::MyTrait`
        // required for `std::ptr::Unique<dyn tests::test_trait2::MyTrait>` to implement `std::marker::Send`
        // context.data::<Box<dyn MyTrait>>();

        // `dyn tests::test_trait2::MyTrait + std::marker::Send` cannot be shared between threads safely
        // the trait `std::marker::Sync` is not implemented for `dyn tests::test_trait2::MyTrait + std::marker::Send`
        // required for `std::ptr::Unique<dyn tests::test_trait2::MyTrait + std::marker::Send>` to implement `std::marker::Sync`
        // context.data::<Box<dyn MyTrait + Send>>();

        let data = context.data::<Box<dyn MyTrait + Send + Sync>>();
        assert!(data.is_none()); // can't get
    }

    #[test]
    fn test_newtyped_boxed_trait() {
        trait MyTrait {
            fn f(&self) -> i32;
        }

        struct MyStruct {
            i: i32,
        }

        impl MyTrait for MyStruct {
            fn f(&self) -> i32 {
                self.i
            }
        }

        struct MyData(Box<dyn MyTrait + Send + Sync>);

        let mut request = Request(HashMap::new());
        request.data(MyData(Box::new(MyStruct { i: 123_i32 })));
        let context = Context(&request);

        let data = context.data::<MyData>();
        assert!(data.is_some());
        assert_eq!(data.unwrap().0.f(), 123_i32);
    }

    #[test]
    fn test_trait_with_associated_types() {
        trait MyTrait {
            type T;
            fn f(&self) -> Self::T;
        }

        struct MyStruct {
            i: i32,
        }

        impl MyTrait for MyStruct {
            type T = i32;
            fn f(&self) -> Self::T {
                self.i
            }
        }

        // specify the associated type: `MyTrait<I = Type>`
        // struct MyData(Box<dyn MyTrait + Send + Sync>);

        struct MyData(Box<dyn MyTrait<T = i32> + Send + Sync>);

        let mut request = Request(HashMap::new());
        request.data(MyData(Box::new(MyStruct { i: 123_i32 })));
        let context = Context(&request);

        let data = context.data::<MyData>();
        assert!(data.is_some());
        assert_eq!(data.unwrap().0.f(), 123_i32);
    }
}
