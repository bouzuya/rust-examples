// <https://docs.rs/axum/0.6.18/axum/index.html#example>
use axum::{routing::get, Router};

fn build_app() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = build_app();

    let addr = "0.0.0.0:3000".parse()?;
    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?)
}
