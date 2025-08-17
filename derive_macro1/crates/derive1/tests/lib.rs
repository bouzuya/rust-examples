#[test]
fn main() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/ui/compile-fail-0.rs");
    test_cases.pass("tests/ui/pass-no-variants.rs");
    test_cases.pass("tests/ui/pass-with-helper-attr.rs");
    test_cases.pass("tests/ui/pass.rs");
}
