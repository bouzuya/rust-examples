use std::{future::Future, pin::Pin};

use google_authz::{Credentials, GoogleAuthz};
use tower::ServiceExt;

type Body = String;

struct MyHttpClient {
    client: reqwest::Client,
}

impl MyHttpClient {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl tower::Service<http::Request<Body>> for MyHttpClient {
    type Response = reqwest::Response;

    type Error = reqwest::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.client.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        let req = reqwest::Request::try_from(req).unwrap();
        Box::pin(self.client.execute(req))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = MyHttpClient::new();
    let credentials = Credentials::builder().no_credentials().build().await?;
    let client = GoogleAuthz::builder(service)
        .credentials(credentials)
        // I can't use enforce_https method
        // .enforce_https(false)
        .build()
        .await;
    let request = http::Request::get("http://example.com").body(Body::default())?;
    let response = client.oneshot(request).await?;
    println!("{:#?}", response);
    println!("{}", response.text().await?);
    Ok(())
}
