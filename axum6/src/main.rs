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
        .merge(private_d())
        .merge(private_e())
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

trait Validator {
    fn validate(&self, token: &str) -> bool;
}

struct ExtractorC;

impl<S> axum::extract::FromRequestParts<S> for ExtractorC
where
    S: Validator + Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            match parts.headers.get(axum::http::header::AUTHORIZATION) {
                Some(header_value) => match header_value.as_bytes().strip_prefix(b"Bearer ") {
                    None => Err(axum::http::StatusCode::FORBIDDEN),
                    Some(token) => {
                        let s = String::from_utf8(token.to_vec())
                            .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;
                        if state.validate(&s) {
                            Ok(ExtractorC)
                        } else {
                            Err(axum::http::StatusCode::FORBIDDEN)
                        }
                    }
                },
                None => Err(axum::http::StatusCode::UNAUTHORIZED),
            }
        }
    }
}

fn private_d() -> axum::Router<()> {
    axum::Router::new()
        .route(
            "/private/d",
            axum::routing::get(private_d_handler::<StateD>),
        )
        .with_state(StateD)
}

async fn private_d_handler<S>(ExtractorC: ExtractorC) -> impl axum::response::IntoResponse {
    "/private/d"
}

#[derive(Clone)]
struct StateD;

impl Validator for StateD {
    fn validate(&self, token: &str) -> bool {
        token == "d"
    }
}

fn private_e() -> axum::Router<()> {
    axum::Router::new()
        .nest(
            "/private/e",
            axum::Router::new()
                .route("/1", axum::routing::get(private_e_1_handler))
                .route("/2", axum::routing::get(private_e_2_handler))
                .route_layer(axum::middleware::from_extractor::<ExtractorB>()),
        )
        .route(
            "/private/e/3",
            axum::routing::get(|| async { "/private/e/3" }),
        )
}

async fn private_e_1_handler() -> impl axum::response::IntoResponse {
    "/private/e/1"
}

async fn private_e_2_handler() -> impl axum::response::IntoResponse {
    "/private/e/2"
}

// async fn private_e_layer(
//     req: axum::http::Request<axum::body::Body>,
//     next: axum::middleware::Next,
// ) -> impl axum::response::IntoResponse {
//     next.run(req).await
// }

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

    #[tokio::test]
    async fn test_private_d() -> anyhow::Result<()> {
        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            // no token
            .uri("/private/d")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/d")
            // invalid token
            .header(axum::http::header::AUTHORIZATION, "Bearer c")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/d")
            // valid token
            .header(axum::http::header::AUTHORIZATION, "Bearer d")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "/private/d");
        Ok(())
    }

    #[tokio::test]
    async fn test_private_e() -> anyhow::Result<()> {
        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/e/1")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/e/1")
            .header(axum::http::header::AUTHORIZATION, "Bearer b")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "/private/e/1");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/e/2")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(response.into_body_string().await?, "");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/e/2")
            .header(axum::http::header::AUTHORIZATION, "Bearer b")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "/private/e/2");

        let app = router();
        let request = axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri("/private/e/3")
            .body(axum::body::Body::empty())?;
        let response = send_request(app, request).await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "/private/e/3");
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
