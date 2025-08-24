fn main() {
    struct MyList(i32, i32);

    impl std::fmt::Debug for MyList {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entry(&self.0).entry(&self.1).finish()
        }
    }

    struct MyMap(i32, i32);

    impl std::fmt::Debug for MyMap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entry(&self.0, &self.1).finish()
        }
    }

    struct MySet(i32, i32);

    impl std::fmt::Debug for MySet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_set().entry(&self.0).entry(&self.1).finish()
        }
    }

    assert_eq!(format!("{:?}", MyList(1, 2)), "[1, 2]");
    assert_eq!(format!("{:?}", MyMap(1, 2)), "{1: 2}");
    assert_eq!(format!("{:?}", MySet(1, 2)), "{1, 2}");
}
