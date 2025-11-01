fn main() {}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_map_request() -> anyhow::Result<()> {
        async fn handler(header_map: axum::http::HeaderMap) -> String {
            format!("X-NAME: {:?}", header_map.get("X-NAME"))
        }

        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get(handler))
            .layer(tower::ServiceBuilder::new().map_request(
                |mut request: axum::http::Request<axum::body::Body>| {
                    request
                        .headers_mut()
                        .insert("X-NAME", axum::http::HeaderValue::from_static("bouzuya"));
                    request
                },
            ));
        let response = send_request(
            app,
            axum::http::Request::builder()
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_string().await?,
            "X-NAME: Some(\"bouzuya\")"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_map_response() -> anyhow::Result<()> {
        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get("Original Body"))
            .layer(tower::ServiceBuilder::new().map_response(
                |mut response: axum::http::Response<axum::body::Body>| {
                    *response.body_mut() = axum::body::Body::from("New Body");
                    response
                },
            ));
        let response = send_request(
            app,
            axum::http::Request::builder()
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "New Body");
        Ok(())
    }

    #[tokio::test]
    async fn test_then() -> anyhow::Result<()> {
        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get("Original Body"))
            .layer(tower::ServiceBuilder::new().then(
                |result: Result<
                    axum::response::Response<axum::body::Body>,
                    std::convert::Infallible,
                >| async {
                    match result {
                        Ok(mut response) => {
                            *response.body_mut() = axum::body::Body::from("New Body");
                            Ok(response)
                        }
                        Err(e) => Err(e),
                    }
                },
            ));
        let response = send_request(
            app,
            axum::http::Request::builder()
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "New Body");
        Ok(())
    }

    #[tokio::test]
    async fn test_and_then() -> anyhow::Result<()> {
        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get("Original Body"))
            .layer(tower::ServiceBuilder::new().and_then(
                |mut response: axum::response::Response<axum::body::Body>| async move {
                    *response.body_mut() = axum::body::Body::from("New Body");
                    Result::<
                                axum::response::Response<axum::body::Body>,
                                std::convert::Infallible,
                            >::Ok(response)
                },
            ));
        let response = send_request(
            app,
            axum::http::Request::builder()
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "New Body");
        Ok(())
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
