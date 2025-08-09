#[test]
fn test() {
    assert_eq!(function_like::function_like!("Hello"), "Hello");
    assert_eq!(function_like::function_like!("abc"), "abc");
}

#[test]
fn test_trybuild() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/ui/compile_fail.rs");
}
