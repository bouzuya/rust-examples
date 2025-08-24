mod other {
    #[derive(Debug)]
    pub struct Id(pub String);
}

mod example1 {
    #[derive(Debug)]
    pub struct Id(pub super::other::Id);
}

mod example2 {
    pub struct Id(pub super::other::Id);
    impl std::fmt::Debug for Id {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
}

fn main() {
    assert_eq!(
        format!("{:?}", other::Id("abc123".to_owned())),
        "Id(\"abc123\")"
    );

    assert_eq!(
        format!("{:?}", example1::Id(other::Id("abc123".to_owned()))),
        "Id(Id(\"abc123\"))"
    );

    assert_eq!(
        format!("{:?}", example2::Id(other::Id("abc123".to_owned()))),
        "Id(\"abc123\")"
    );
}
