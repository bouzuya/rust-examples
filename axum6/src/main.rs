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

fn private_router() -> axum::Router<()> {
    axum::Router::new()
        .merge(private_a())
        .merge(private_b())
        .merge(private_c())
}

struct ExtractorA;

impl axum::extract::FromRequestParts<()> for ExtractorA {
    type Rejection = axum::http::StatusCode;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &(),
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            match parts.headers.get(axum::http::header::AUTHORIZATION) {
                Some(header_value) => {
                    // 本来 token は base64 encoded
                    if header_value.as_bytes() == b"Bearer a" {
                        Ok(ExtractorA)
                    } else {
                        Err(axum::http::StatusCode::FORBIDDEN)
                    }
                }
                None => Err(axum::http::StatusCode::UNAUTHORIZED),
            }
        }
    }
}

fn private_a() -> axum::Router<()> {
    axum::Router::new()
        .route("/private/a", axum::routing::get(|| async { "/private/a" }))
        .route_layer(axum::middleware::from_extractor::<ExtractorA>())
}

struct ExtractorB;

impl axum::extract::FromRequestParts<()> for ExtractorB {
    type Rejection = axum::http::StatusCode;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &(),
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            match parts.headers.get(axum::http::header::AUTHORIZATION) {
                Some(header_value) => {
                    // 本来 token は base64 encoded
                    if header_value.as_bytes() == b"Bearer b" {
                        Ok(ExtractorB)
                    } else {
                        Err(axum::http::StatusCode::FORBIDDEN)
                    }
                }
                None => Err(axum::http::StatusCode::UNAUTHORIZED),
            }
        }
    }
}

fn private_b() -> axum::Router<()> {
    axum::Router::new()
        .route("/private/b", axum::routing::get(|| async { "/private/b" }))
        .route_layer(axum::middleware::from_extractor::<ExtractorB>())
}

fn private_c() -> axum::Router<()> {
    axum::Router::new().route("/private/c", axum::routing::get(private_c_handler))
}

async fn private_c_handler(ExtractorB: ExtractorB) -> impl axum::response::IntoResponse {
    "/private/c"
}

fn router() -> axum::Router<()> {
    axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/users", axum::routing::post(create_user))
        .route("/users/{user_id}", axum::routing::get(get_user))
        .merge(private_router())
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
    async fn test_private_a() -> anyhow::Result<()> {
        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            // no token
            .uri("/private/a")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/a")
            // invalid token
            .header(axum::http::header::AUTHORIZATION, "Bearer b")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/a")
            // valid token
            .header(axum::http::header::AUTHORIZATION, "Bearer a")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "/private/a");
        Ok(())
    }

    #[tokio::test]
    async fn test_private_b() -> anyhow::Result<()> {
        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            // no token
            .uri("/private/b")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/b")
            // invalid token
            .header(axum::http::header::AUTHORIZATION, "Bearer a")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/b")
            // valid token
            .header(axum::http::header::AUTHORIZATION, "Bearer b")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "/private/b");
        Ok(())
    }

    #[tokio::test]
    async fn test_private_c() -> anyhow::Result<()> {
        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            // no token
            .uri("/private/c")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/c")
            // invalid token
            .header(axum::http::header::AUTHORIZATION, "Bearer a")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/c")
            // valid token
            .header(axum::http::header::AUTHORIZATION, "Bearer b")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "/private/c");
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
