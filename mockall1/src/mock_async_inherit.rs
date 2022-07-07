use async_trait::async_trait;

trait A {
    fn a(&self, x: i32) -> i32 {
        x + 1
    }
}

#[async_trait]
trait B: A {
    async fn b(&self, b: bool) -> bool {
        !b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mockall::mock! {
        C {}
        impl A for C {
        }
        #[async_trait]
        impl B for C {
            async fn b(&self, b: bool) -> bool;
        }
    }

    #[tokio::test]
    async fn test() {
        let mut mock = MockC::new();
        mock.expect_b().times(2).returning(|b| b);
        assert!(mock.b(true).await);
        assert!(!mock.b(false).await);
    }
}
