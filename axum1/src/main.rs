// <https://docs.rs/axum/0.6.18/axum/index.html#example>
// <https://docs.rs/axum/0.6.18/axum/index.html#routing>
// <https://docs.rs/axum/0.6.18/axum/index.html#extractors>
// <https://docs.rs/axum/0.6.18/axum/index.html#responses>
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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
async fn extractors_path(Path(user_id): Path<u32>) -> String {
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
async fn extractors_query(Query(params): Query<HashMap<String, String>>) -> String {
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
async fn extractors_json(Json(payload): Json<serde_json::Value>) -> String {
    format!("{:?}", payload)
}

// ```console
// $ curl -D - 'http://localhost:3000/responses/plain_text'
// HTTP/1.1 200 OK
// content-type: text/plain; charset=utf-8
// content-length: 3
// date: Sat, 27 May 2023 23:50:33 GMT
//
// foo
// ```
async fn responses_plain_text() -> &'static str {
    "foo"
}

// ```console
// $ curl -D - 'http://localhost:3000/responses/json'
// HTTP/1.1 200 OK
// content-type: application/json
// content-length: 11
// date: Sat, 27 May 2023 23:51:15 GMT
//
// {"data":42}
// ```
async fn responses_json() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "data": 42 }))
}

struct AppState {
    counter: i32,
}

type SharedState = Arc<Mutex<AppState>>;

async fn shared_state_increment_count(State(state): State<SharedState>) -> Result<(), StatusCode> {
    let mut state = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    state.counter += 1;
    Ok(())
}

async fn shared_state_get_count(State(state): State<SharedState>) -> Result<String, StatusCode> {
    let state = state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(state.counter.to_string())
}

fn build_app(state: SharedState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/extractors/path/:id", get(extractors_path))
        .route("/extractors/query", get(extractors_query))
        .route("/extractors/json", post(extractors_json))
        .route("/responses/plain_text", get(responses_plain_text))
        .route("/responses/json", get(responses_json))
        .route(
            "/shared_state/count/increment",
            put(shared_state_increment_count),
        )
        .route("/shared_state/count", get(shared_state_get_count))
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = Arc::new(Mutex::new(AppState { counter: 0 }));
    let app = build_app(state);

    let addr = "0.0.0.0:3000".parse()?;
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?)
}
