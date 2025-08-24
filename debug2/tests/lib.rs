#[test]
fn test() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/ui/compile_fail_non_debug_field.rs");
    test_cases.pass("tests/ui/pass_custom.rs");
    test_cases.pass("tests/ui/pass_masked_field.rs");
    test_cases.pass("tests/ui/pass_non_debug_field.rs");
    test_cases.pass("tests/ui/pass_wrapped.rs");
}
