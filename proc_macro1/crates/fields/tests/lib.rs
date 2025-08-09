#[test]
fn test1() {
    fields::fields!(field1 = i32);
    let _ = Struct1 { field1: 42 };
}

#[test]
fn test() {
    let test_cases = trybuild::TestCases::new();
    test_cases.pass("tests/ui/pass.rs");
    test_cases.compile_fail("tests/ui/compile_fail.rs");
}
