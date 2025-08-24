fn main() {
    #[derive(Debug)]
    struct Input1 {
        email: String,
        password: String,
    }
    assert_eq!(
        format!(
            "{:?}",
            Input1 {
                email: "happy@example.com".to_owned(),
                password: "password".to_owned()
            }
        ),
        "Input1 { email: \"happy@example.com\", password: \"password\" }"
    );

    struct Input2 {
        email: String,
        password: String,
    }
    impl std::fmt::Debug for Input2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Input2")
                .field("email", &self.email)
                .field("password", &"********")
                .finish()
        }
    }
    assert_eq!(
        format!(
            "{:?}",
            Input2 {
                email: "happy@example.com".to_owned(),
                password: "password".to_owned()
            }
        ),
        "Input2 { email: \"happy@example.com\", password: \"********\" }"
    );
}
