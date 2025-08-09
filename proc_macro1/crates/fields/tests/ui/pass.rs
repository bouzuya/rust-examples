fn main() {
    fields::fields!(field1 = i32, field2 = String);
    let _ = Struct1 {
        field1: 42,
        field2: "Hello".to_owned(),
    };
}
