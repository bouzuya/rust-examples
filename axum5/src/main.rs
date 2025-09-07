async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_user(
    axum::extract::Path(user_id): axum::extract::Path<u32>,
) -> Result<String, axum::http::StatusCode> {
    if user_id == 0 {
        Err(axum::http::StatusCode::NOT_FOUND)
    } else {
        Ok(format!("User {}", user_id))
    }
}

async fn create_user(
    axum::extract::Json(json): axum::extract::Json<serde_json::Value>,
) -> axum::response::Json<serde_json::Value> {
    // ...
    axum::response::Json(json)
}

async fn fallback_handler() -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::OK, "fallback")
}

async fn method_not_allowed_fallback() -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::OK, "method_not_allowed_fallback")
}

fn router() -> axum::Router<()> {
    axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/users", axum::routing::post(create_user))
        .route("/users/{user_id}", axum::routing::get(get_user))
        .fallback(fallback_handler)
        .method_not_allowed_fallback(method_not_allowed_fallback)
}

#[tokio::main]
async fn main() {
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_root() -> anyhow::Result<()> {
        let router = router();
        let request = axum::http::Request::builder()
            .uri("/")
            .body(axum::body::Body::empty())?;
        let response = send_request(router, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "Hello, world!");
        Ok(())
    }

    #[tokio::test]
    async fn test_create_user() -> anyhow::Result<()> {
        let router = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri("/users")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(r#"{"name":"John Doe"}"#))?;
        let response = send_request(router, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, r#"{"name":"John Doe"}"#);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user() -> anyhow::Result<()> {
        let router = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/users/1")
            .body(axum::body::Body::empty())?;
        let response = send_request(router, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, r#"User 1"#);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_not_found() -> anyhow::Result<()> {
        let router = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/users/0")
            .body(axum::body::Body::empty())?;
        let response = send_request(router, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        assert_eq!(response.into_body_string().await?, "");
        Ok(())
    }

    #[tokio::test]
    async fn test_path_not_found() -> anyhow::Result<()> {
        let router = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/unknown")
            .body(axum::body::Body::empty())?;
        let response = send_request(router, request).await?;
        // no fallback => HTTP 404
        // assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        // assert_eq!(response.into_body_string().await?, "");
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "fallback");
        Ok(())
    }

    #[tokio::test]
    async fn test_method_not_allowed() -> anyhow::Result<()> {
        let router = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri("/")
            .body(axum::body::Body::empty())?;
        let response = send_request(router, request).await?;
        // no fallback => HTTP 405
        // assert_eq!(
        //     response.status(),
        //     axum::http::StatusCode::METHOD_NOT_ALLOWED
        // );
        // assert_eq!(response.into_body_string().await?, "");
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_string().await?,
            "method_not_allowed_fallback"
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
