async fn path(axum::extract::Path(_user_id): axum::extract::Path<u32>) {}

async fn query(
    axum::extract::Query(_params): axum::extract::Query<std::collections::HashMap<String, String>>,
) {
}

async fn json(axum::extract::Json(_payload): axum::extract::Json<serde_json::Value>) {}

fn route() -> axum::Router<()> {
    axum::Router::new()
        .route("/path/{user_id}", axum::routing::get(path))
        .route("/query", axum::routing::get(query))
        .route("/json", axum::routing::get(json))
}

#[tokio::main]
async fn main() {
    let app = route();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
