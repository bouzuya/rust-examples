fn main() {}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
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
