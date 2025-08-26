async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}

fn route() -> axum::Router<()> {
    axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/foo", axum::routing::get(get_foo).post(post_foo))
        .route("/foo/bar", axum::routing::get(foo_bar))
}

#[tokio::main]
async fn main() {
    let app = route();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
