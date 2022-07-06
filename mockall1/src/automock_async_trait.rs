use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
trait Bar {
    async fn bar(&self, x: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use mockall::{predicate::eq, PredicateBooleanExt};

    use super::*;

    #[tokio::test]
    async fn test() {
        let mut mock = MockBar::new();
        mock.expect_bar()
            .with(eq(4).or(eq(3)))
            .times(2)
            .returning(|x| Box::pin(async move { x % 2 == 0 }));
        assert!(mock.bar(4).await);
        assert!(!mock.bar(3).await);
    }
}
