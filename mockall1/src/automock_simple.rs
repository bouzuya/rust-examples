use mockall::automock;

#[automock]
trait Foo {
    fn foo(&self, x: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test() {
        let mut mock = MockFoo::new();
        mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
        assert_eq!(5, mock.foo(4));
    }
}
