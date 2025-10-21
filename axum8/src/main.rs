#[tokio::main]
async fn main() {
    async fn handler() {}

    async fn layer_one(
        request: axum::http::Request<axum::body::Body>,
        next: axum::middleware::Next,
    ) -> axum::response::Response {
        println!("layer one start");
        let response = next.run(request).await;
        println!("layer one end");
        response
    }

    async fn layer_two(
        request: axum::http::Request<axum::body::Body>,
        next: axum::middleware::Next,
    ) -> axum::response::Response {
        println!("layer two start");
        let response = next.run(request).await;
        println!("layer two end");
        response
    }

    async fn layer_three(
        request: axum::http::Request<axum::body::Body>,
        next: axum::middleware::Next,
    ) -> axum::response::Response {
        println!("layer three start");
        let response = next.run(request).await;
        println!("layer three end");
        response
    }

    let app = axum::Router::<()>::new()
        .route("/", axum::routing::get(handler))
        .layer(axum::middleware::from_fn(layer_three))
        .layer(axum::middleware::from_fn(layer_two))
        .layer(axum::middleware::from_fn(layer_one));
    let _response = send_request(
        app,
        axum::http::Request::builder()
            .method("GET")
            .uri("/")
            .body(axum::body::Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
    // =>
    // layer one start
    // layer two start
    // layer three start
    // layer three end
    // layer two end
    // layer one end

    println!("---");

    let app = axum::Router::<()>::new()
        .route("/", axum::routing::get(handler))
        .layer(
            tower::ServiceBuilder::new()
                .layer(axum::middleware::from_fn(layer_one))
                .layer(axum::middleware::from_fn(layer_two))
                .layer(axum::middleware::from_fn(layer_three)),
        );
    let _response = send_request(
        app,
        axum::http::Request::builder()
            .method("GET")
            .uri("/")
            .body(axum::body::Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
    // =>
    // layer one start
    // layer two start
    // layer three start
    // layer three end
    // layer two end
    // layer one end
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
