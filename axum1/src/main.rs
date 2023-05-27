// <https://docs.rs/axum/0.6.18/axum/index.html#example>
// <https://docs.rs/axum/0.6.18/axum/index.html#routing>
// <https://docs.rs/axum/0.6.18/axum/index.html#extractors>
use axum::{
    extract::{Json, Path, Query},
    routing::{get, post},
    Router,
};
use std::collections::HashMap;

async fn root() -> &'static str {
    "Hello, World!"
}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}

// ```console
// $ curl -D - 'http://localhost:3000/extractors/path/foo'
// HTTP/1.1 400 Bad Request
// content-type: text/plain; charset=utf-8
// content-length: 44
// date: Sat, 27 May 2023 23:39:44 GMT
//
// Invalid URL: Cannot parse `"foo"` to a `u32`
//
// $ curl -D - 'http://localhost:3000/extractors/path/123'
// HTTP/1.1 200 OK
// content-type: text/plain; charset=utf-8
// content-length: 3
// date: Sat, 27 May 2023 23:40:01 GMT
//
// 123
// ```
async fn path(Path(user_id): Path<u32>) -> String {
    format!("{:?}", user_id)
}

// ```console
// $ curl -D - 'http://localhost:3000/extractors/query?foo=123'
// HTTP/1.1 200 OK
// content-type: text/plain; charset=utf-8
// content-length: 14
// date: Sat, 27 May 2023 23:40:30 GMT
//
// {"foo": "123"}
//
// $ curl -D - 'http://localhost:3000/extractors/query'
// HTTP/1.1 200 OK
// content-type: text/plain; charset=utf-8
// content-length: 2
// date: Sat, 27 May 2023 23:40:51 GMT
//
// {}
// ```
async fn query(Query(params): Query<HashMap<String, String>>) -> String {
    format!("{:?}", params)
}

// ```console
// $ curl -D - -d '{"foo":"123"}' 'http://localhost:3000/extractors/json'
// HTTP/1.1 415 Unsupported Media Type
// content-type: text/plain; charset=utf-8
// content-length: 54
// date: Sat, 27 May 2023 23:41:46 GMT
//
// Expected request with `Content-Type: application/json`
//
// $ curl -D - -H 'Content-Type: application/json' -d '{"foo":"123"}' 'http://localhost:3000/extractors/json'
// HTTP/1.1 200 OK
// content-type: text/plain; charset=utf-8
// content-length: 29
// date: Sat, 27 May 2023 23:42:03 GMT
//
// Object {"foo": String("123")}
// ```
async fn json(Json(payload): Json<serde_json::Value>) -> String {
    format!("{:?}", payload)
}

fn build_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/extractors/path/:id", get(path))
        .route("/extractors/query", get(query))
        .route("/extractors/json", post(json))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = build_app();

    let addr = "0.0.0.0:3000".parse()?;
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?)
}
