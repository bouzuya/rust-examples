trait A {
    fn a(&self, x: i32) -> i32 {
        x + 1
    }
}

trait B: A {
    fn b(&self, b: bool) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    mockall::mock! {
        C {}
        impl A for C {
        }
        impl B for C {
            fn b(&self, b: bool) -> bool;
        }
    }

    #[tokio::test]
    async fn test() {
        let mut mock = MockC::new();
        mock.expect_b().times(2).returning(|b| b);
        assert!(mock.b(true));
        assert!(!mock.b(false));
    }
}
