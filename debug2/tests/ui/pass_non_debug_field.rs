struct NonDebug;

struct MyDebug {
    field: NonDebug,
}

impl std::fmt::Debug for MyDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyDebug")
            .field("field", &"NonDebug")
            .finish()
    }
}

fn main() {
    assert_eq!(
        format!("{:?}", MyDebug { field: NonDebug }),
        "MyDebug { field: \"NonDebug\" }"
    );
}
