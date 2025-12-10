fn main() {
    #[derive(serde::Serialize)]
    struct T {
        q: Option<String>,
    }
    assert_eq!(
        serde_urlencoded::to_string(&T {
            q: Some("test".to_owned())
        })
        .unwrap(),
        "q=test"
    );
    assert_eq!(
        serde_urlencoded::to_string(&T {
            q: Some("".to_owned())
        })
        .unwrap(),
        "q="
    );
    assert_eq!(serde_urlencoded::to_string(&T { q: None }).unwrap(), "");
}
