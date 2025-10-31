#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::{http::StatusCode, response::IntoResponse};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        async fn handler(header_map: axum::http::HeaderMap) -> String {
            format!("X-NAME: {:?}", header_map.get("X-NAME"))
        }

        async fn my_middleware_fn(
            // axum::http::HeaderMap は FromRequestParts の実装の例
            header_map: axum::http::HeaderMap,
            // axum::http::Request は FromRequest の実装の例
            // 通常、 Next::run が Request を要求するので、自身で構築しない限りは Request を受け取ることになる
            mut request: axum::http::Request<axum::body::Body>,
            next: axum::middleware::Next,
        ) -> axum::response::Response<axum::body::Body> {
            if header_map.contains_key("X-NAME") {
                return StatusCode::BAD_REQUEST.into_response();
            }

            request
                .headers_mut()
                .insert("X-NAME", axum::http::HeaderValue::from_static("bouzuya"));
            let response = next.run(request).await;
            response
        }

        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get(handler))
            .layer(axum::middleware::from_fn(my_middleware_fn));
        let response = send_request(
            app,
            axum::http::Request::builder()
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.into_body_string().await?,
            "X-NAME: Some(\"bouzuya\")"
        );

        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get(handler))
            .layer(axum::middleware::from_fn(my_middleware_fn));
        let response = send_request(
            app,
            axum::http::Request::builder()
                .header("X-NAME", "dummy")
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(response.into_body_string().await?, "");
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
