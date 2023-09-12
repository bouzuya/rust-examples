fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test1() {
        assert_eq!(2 + 2, 5);
    }

    #[test]
    #[should_panic]
    fn test2() {
        let result = std::panic::catch_unwind(|| {
            assert_eq!(2 + 2, 5);
        });
        assert!(result.is_ok());
    }

    fn test3_setup() {
        println!("test3 setup")
    }

    fn test3_teardown() {
        println!("test3 teardown");
    }

    fn run_test3<F, R>(f: F)
    where
        F: FnOnce() -> R + std::panic::UnwindSafe,
    {
        test3_setup();
        let result = std::panic::catch_unwind(f);
        test3_teardown();
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic]
    fn test3() {
        run_test3(|| {
            assert_eq!(2 + 2, 5);
        });
    }

    #[tokio::test]
    #[should_panic]
    async fn test4() {
        async {
            assert_eq!(2 + 2, 5);
        }
        .await
    }

    #[tokio::test]
    #[should_panic]
    async fn test5() {
        let result = futures::FutureExt::catch_unwind(async {
            assert_eq!(2 + 2, 5);
        })
        .await;
        assert!(result.is_ok());
    }

    async fn test6_setup() {
        println!("test6 setup")
    }

    async fn test6_teardown() {
        println!("test6 teardown");
    }

    async fn run_test6<R>(
        f: impl std::future::Future<Output = R> + Sized + std::panic::UnwindSafe,
    ) {
        test6_setup().await;
        let result = futures::FutureExt::catch_unwind(f).await;
        test6_teardown().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[should_panic]
    async fn test6() {
        run_test6(async {
            assert_eq!(2 + 2, 5);
        })
        .await
    }

    async fn test7_setup() -> anyhow::Result<()> {
        println!("test7 setup");
        Ok(())
    }

    async fn test7_teardown() -> anyhow::Result<()> {
        println!("test7 teardown");
        Ok(())
    }

    async fn run_test7(
        f: impl std::future::Future<Output = anyhow::Result<()>> + Sized + std::panic::UnwindSafe,
    ) -> anyhow::Result<()> {
        test7_setup().await?;
        let result = futures::FutureExt::catch_unwind(f).await;
        test7_teardown().await?;
        result.unwrap()
    }

    #[tokio::test]
    async fn test7() -> anyhow::Result<()> {
        run_test7(async {
            assert_eq!(2 + 2, 4);
            Ok(())
        })
        .await
    }

    async fn test8_setup() -> anyhow::Result<()> {
        println!("test8 setup");
        Ok(())
    }

    async fn test8_teardown() -> anyhow::Result<()> {
        println!("test8 teardown");
        Ok(())
    }

    async fn run_test8<F, FutS, FutT, S, T>(f: F, setup: S, teardown: T) -> anyhow::Result<()>
    where
        F: std::future::Future<Output = anyhow::Result<()>> + Sized + std::panic::UnwindSafe,
        FutS: std::future::Future<Output = anyhow::Result<()>>,
        FutT: std::future::Future<Output = anyhow::Result<()>>,
        S: Fn() -> FutS,
        T: Fn() -> FutT,
    {
        setup().await?;
        let result = futures::FutureExt::catch_unwind(f).await;
        teardown().await?;
        result.unwrap()
    }

    #[tokio::test]
    async fn test8() -> anyhow::Result<()> {
        run_test8(
            async {
                assert_eq!(2 + 2, 4);
                Ok(())
            },
            test8_setup,
            test8_teardown,
        )
        .await
    }
}
