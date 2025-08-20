#[derive(Debug)]
struct Unit;

#[derive(Debug)]
struct Tuple1(());

#[derive(Debug)]
struct Tuple2(bool, i32);

#[derive(Debug)]
struct Struct1 {
    s: String,
}

#[derive(Debug)]
struct Struct2 {
    nested: Struct1,
}

#[derive(Debug)]
enum Enum {
    Unit,
    Tuple(bool),
    Struct { n: i32 },
}

fn main() {
    let s = Unit;
    assert_eq!(format!("{:?}", s), "Unit");

    let s = Tuple1(());
    assert_eq!(format!("{:?}", s), "Tuple1(())");

    let s = Tuple2(true, 123);
    assert_eq!(format!("{:?}", s), "Tuple2(true, 123)");

    let s = Struct1 {
        s: "abc".to_owned(),
    };
    assert_eq!(format!("{:?}", s), "Struct1 { s: \"abc\" }");

    let s = Struct2 { nested: s };
    assert_eq!(
        format!("{:?}", s),
        "Struct2 { nested: Struct1 { s: \"abc\" } }"
    );

    let s = Enum::Unit;
    assert_eq!(format!("{:?}", s), "Unit");
    let s = Enum::Tuple(true);
    assert_eq!(format!("{:?}", s), "Tuple(true)");
    let s = Enum::Struct { n: 123 };
    assert_eq!(format!("{:?}", s), "Struct { n: 123 }");
}
