fn main() {}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_merge() -> anyhow::Result<()> {
        fn router_a() -> axum::Router<()> {
            axum::Router::new()
                .route("/a/1", axum::routing::get(|| async { "a_1" }))
                .route("/a/2", axum::routing::get(|| async { "a_2" }))
        }

        fn router_b() -> axum::Router<()> {
            axum::Router::new()
                .route("/b/1", axum::routing::get(|| async { "b_1" }))
                .route("/b/2", axum::routing::get(|| async { "b_2" }))
        }

        fn router() -> axum::Router<()> {
            axum::Router::new().merge(router_a()).merge(router_b())
        }

        async fn f(path: &str, status: axum::http::StatusCode, body: &str) -> anyhow::Result<()> {
            let router = router();
            let request = axum::http::Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())?;
            let response = send_request(router, request).await?;
            assert_eq!(response.status(), status);
            assert_eq!(response.into_body_string().await?, body);
            Ok(())
        }
        f("/a/1", axum::http::StatusCode::OK, "a_1").await?;
        f("/a/2", axum::http::StatusCode::OK, "a_2").await?;
        f("/b/1", axum::http::StatusCode::OK, "b_1").await?;
        f("/b/2", axum::http::StatusCode::OK, "b_2").await?;
        f("/c/1", axum::http::StatusCode::NOT_FOUND, "").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_nest() -> anyhow::Result<()> {
        fn router_a() -> axum::Router<()> {
            axum::Router::new()
                .route("/1", axum::routing::get(|| async { "a_1" }))
                .route("/2", axum::routing::get(|| async { "a_2" }))
        }

        fn router_b() -> axum::Router<()> {
            axum::Router::new()
                .route("/1", axum::routing::get(|| async { "b_1" }))
                .route("/2", axum::routing::get(|| async { "b_2" }))
        }

        fn router() -> axum::Router<()> {
            axum::Router::new()
                .nest("/a", router_a())
                .nest("/b", router_b())
        }

        async fn f(path: &str, status: axum::http::StatusCode, body: &str) -> anyhow::Result<()> {
            let router = router();
            let request = axum::http::Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())?;
            let response = send_request(router, request).await?;
            assert_eq!(response.status(), status);
            assert_eq!(response.into_body_string().await?, body);
            Ok(())
        }
        f("/a/1", axum::http::StatusCode::OK, "a_1").await?;
        f("/a/2", axum::http::StatusCode::OK, "a_2").await?;
        f("/b/1", axum::http::StatusCode::OK, "b_1").await?;
        f("/b/2", axum::http::StatusCode::OK, "b_2").await?;
        f("/c/1", axum::http::StatusCode::NOT_FOUND, "").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_merge_with_state() -> anyhow::Result<()> {
        #[derive(Clone)]
        struct StateA {
            a: String,
        }

        fn router_a() -> axum::Router<StateA> {
            axum::Router::new()
                .route("/a/1", axum::routing::get(|axum::extract::State(StateA { a }): axum::extract::State<StateA>| async move { a }))
        }

        #[derive(Clone)]
        struct StateB {
            b: String,
        }

        fn router_b() -> axum::Router<StateB> {
            axum::Router::new()
                .route("/b/1", axum::routing::get(|axum::extract::State(StateB { b }): axum::extract::State<StateB>| async move { b }))
        }

        fn router() -> axum::Router<()> {
            axum::Router::new()
                .merge(router_a().with_state(StateA {
                    a: "a_1".to_owned(),
                }))
                .merge(router_b().with_state(StateB {
                    b: "b_1".to_owned(),
                }))
        }

        async fn f(path: &str, status: axum::http::StatusCode, body: &str) -> anyhow::Result<()> {
            let router = router();
            let request = axum::http::Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())?;
            let response = send_request(router, request).await?;
            assert_eq!(response.status(), status);
            assert_eq!(response.into_body_string().await?, body);
            Ok(())
        }
        f("/a/1", axum::http::StatusCode::OK, "a_1").await?;
        f("/b/1", axum::http::StatusCode::OK, "b_1").await?;
        f("/c/1", axum::http::StatusCode::NOT_FOUND, "").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_nest_with_state() -> anyhow::Result<()> {
        #[derive(Clone)]
        struct StateA {
            a: String,
        }

        fn router_a() -> axum::Router<StateA> {
            axum::Router::new()
                .route("/1", axum::routing::get(|axum::extract::State(StateA { a }): axum::extract::State<StateA>| async move { a }))
        }

        #[derive(Clone)]
        struct StateB {
            b: String,
        }

        fn router_b() -> axum::Router<StateB> {
            axum::Router::new()
                .route("/1", axum::routing::get(|axum::extract::State(StateB { b }): axum::extract::State<StateB>| async move { b }))
        }

        fn router() -> axum::Router<()> {
            axum::Router::new()
                .nest(
                    "/a",
                    router_a().with_state(StateA {
                        a: "a_1".to_owned(),
                    }),
                )
                .nest(
                    "/b",
                    router_b().with_state(StateB {
                        b: "b_1".to_owned(),
                    }),
                )
        }

        async fn f(path: &str, status: axum::http::StatusCode, body: &str) -> anyhow::Result<()> {
            let router = router();
            let request = axum::http::Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())?;
            let response = send_request(router, request).await?;
            assert_eq!(response.status(), status);
            assert_eq!(response.into_body_string().await?, body);
            Ok(())
        }
        f("/a/1", axum::http::StatusCode::OK, "a_1").await?;
        f("/b/1", axum::http::StatusCode::OK, "b_1").await?;
        f("/c/1", axum::http::StatusCode::NOT_FOUND, "").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_merge_with_state_generic() -> anyhow::Result<()> {
        trait StateTraitA {
            fn a(&self) -> String;
        }

        fn router_a<S: StateTraitA + Clone + Send + Sync + 'static>() -> axum::Router<S> {
            axum::Router::new().route(
                "/a/1",
                axum::routing::get(
                    |axum::extract::State(s): axum::extract::State<S>| async move { s.a() },
                ),
            )
        }

        trait StateTraitB {
            fn b(&self) -> String;
        }

        fn router_b<S: StateTraitB + Clone + Send + Sync + 'static>() -> axum::Router<S> {
            axum::Router::new().route(
                "/b/1",
                axum::routing::get(
                    |axum::extract::State(s): axum::extract::State<S>| async move { s.b() },
                ),
            )
        }

        #[derive(Clone)]
        struct State {
            a: String,
            b: String,
        }

        impl StateTraitA for State {
            fn a(&self) -> String {
                self.a.clone()
            }
        }

        impl StateTraitB for State {
            fn b(&self) -> String {
                self.b.clone()
            }
        }

        fn router() -> axum::Router<()> {
            axum::Router::new()
                .merge(router_a())
                .merge(router_b())
                .with_state(State {
                    a: "a_1".to_owned(),
                    b: "b_1".to_owned(),
                })
        }

        async fn f(path: &str, status: axum::http::StatusCode, body: &str) -> anyhow::Result<()> {
            let router = router();
            let request = axum::http::Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())?;
            let response = send_request(router, request).await?;
            assert_eq!(response.status(), status);
            assert_eq!(response.into_body_string().await?, body);
            Ok(())
        }
        f("/a/1", axum::http::StatusCode::OK, "a_1").await?;
        f("/b/1", axum::http::StatusCode::OK, "b_1").await?;
        f("/c/1", axum::http::StatusCode::NOT_FOUND, "").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_nest_with_state_generic() -> anyhow::Result<()> {
        trait StateTraitA {
            fn a(&self) -> String;
        }

        fn router_a<S: StateTraitA + Clone + Send + Sync + 'static>() -> axum::Router<S> {
            axum::Router::new().route(
                "/1",
                axum::routing::get(
                    |axum::extract::State(s): axum::extract::State<S>| async move { s.a() },
                ),
            )
        }

        trait StateTraitB {
            fn b(&self) -> String;
        }

        fn router_b<S: StateTraitB + Clone + Send + Sync + 'static>() -> axum::Router<S> {
            axum::Router::new().route(
                "/1",
                axum::routing::get(
                    |axum::extract::State(s): axum::extract::State<S>| async move { s.b() },
                ),
            )
        }

        #[derive(Clone)]
        struct State {
            a: String,
            b: String,
        }

        impl StateTraitA for State {
            fn a(&self) -> String {
                self.a.clone()
            }
        }

        impl StateTraitB for State {
            fn b(&self) -> String {
                self.b.clone()
            }
        }

        fn router() -> axum::Router<()> {
            axum::Router::new()
                .nest("/a", router_a())
                .nest("/b", router_b())
                .with_state(State {
                    a: "a_1".to_owned(),
                    b: "b_1".to_owned(),
                })
        }

        async fn f(path: &str, status: axum::http::StatusCode, body: &str) -> anyhow::Result<()> {
            let router = router();
            let request = axum::http::Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())?;
            let response = send_request(router, request).await?;
            assert_eq!(response.status(), status);
            assert_eq!(response.into_body_string().await?, body);
            Ok(())
        }
        f("/a/1", axum::http::StatusCode::OK, "a_1").await?;
        f("/b/1", axum::http::StatusCode::OK, "b_1").await?;
        f("/c/1", axum::http::StatusCode::NOT_FOUND, "").await?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Overlapping method route. Handler for `GET /a/1` already exists")]
    fn test_merge_duplicate() {
        fn router_a() -> axum::Router<()> {
            axum::Router::new().route("/a/1", axum::routing::get(|| async { "merge a_1" }))
        }

        fn router() -> axum::Router<()> {
            axum::Router::new().merge(router_a()).merge(router_a())
        }

        let _ = router();
    }

    #[test]
    #[should_panic(expected = "Overlapping method route. Handler for `GET /a/1` already exists")]
    fn test_nest_duplicate() {
        fn router_a() -> axum::Router<()> {
            axum::Router::new().route("/1", axum::routing::get(|| async { "nest a_1" }))
        }

        fn router() -> axum::Router<()> {
            axum::Router::new()
                .route("/a/1", axum::routing::get(|| async { "a_1" }))
                .nest("/a", router_a())
        }

        let _ = router();
    }

    #[test]
    fn test_merge_empty() {
        let _ = axum::Router::<()>::new().merge(axum::Router::new());
    }

    #[test]
    fn test_nest_empty() {
        let _ = axum::Router::<()>::new().nest("/a", axum::Router::new());
    }

    #[test]
    #[should_panic(expected = "Invalid route: nested routes cannot contain wildcards (*)")]
    fn test_wildcard() {
        let _ = axum::Router::<()>::new().nest("/{*wildcard}", axum::Router::new());
    }

    async fn send_request(
        router: axum::Router<()>,
        request: axum::http::Request<axum::body::Body>,
    ) -> anyhow::Result<axum::response::Response<axum::body::Body>> {
        let response = tower::ServiceExt::oneshot(router, request).await?;
        Ok(response)
    }

    trait ResponseExt {
        async fn into_body_string(self) -> anyhow::Result<String>;
    }

    impl ResponseExt for axum::response::Response<axum::body::Body> {
        async fn into_body_string(self) -> anyhow::Result<String> {
            let bytes = axum::body::to_bytes(self.into_body(), usize::MAX).await?;
            Ok(String::from_utf8(bytes.to_vec())?)
        }
    }
}
