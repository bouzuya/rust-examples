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

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/users", axum::routing::post(create_user))
        .route("/users/{user_id}", axum::routing::get(get_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// $ curl -D - localhost:3000/
// HTTP/1.1 200 OK
// content-type: text/plain; charset=utf-8
// content-length: 13
// date: Thu, 28 Aug 2025 12:53:00 GMT
//
// Hello, world!
//
// $ curl -D - localhost:3000/users/0
// HTTP/1.1 404 Not Found
// content-length: 0
// date: Thu, 28 Aug 2025 12:53:09 GMT
//
//
// $ curl -D - localhost:3000/users/123
// HTTP/1.1 200 OK
// content-type: text/plain; charset=utf-8
// content-length: 8
// date: Thu, 28 Aug 2025 12:53:12 GMT
//
// User 123
//
// $ curl -D - --json '{"name":"bouzuya"}' localhost:3000/users
// HTTP/1.1 200 OK
// content-type: application/json
// content-length: 18
// date: Thu, 28 Aug 2025 12:53:31 GMT
//
// {"name":"bouzuya"}
