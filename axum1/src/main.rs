// <https://docs.rs/axum/0.6.18/axum/index.html#example>
// <https://docs.rs/axum/0.6.18/axum/index.html#routing>
use axum::{routing::get, Router};

async fn root() -> &'static str {
    "Hello, World!"
}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}

fn build_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = build_app();

    let addr = "0.0.0.0:3000".parse()?;
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?)
}
