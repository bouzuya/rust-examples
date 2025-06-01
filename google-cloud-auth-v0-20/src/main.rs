#[tokio::main]
async fn main() {
    let project_id = std::env::var("PROJECT_ID").unwrap();

    let credentials: google_cloud_auth::credentials::Credentials =
        google_cloud_auth::credentials::Builder::default()
            .build()
            .unwrap();
    let header_map: http::HeaderMap = credentials.headers(http::Extensions::new()).await.unwrap();

    let client: reqwest::Client = reqwest::ClientBuilder::new().build().unwrap();
    let response: reqwest::Response = client
        .get(format!(
            "https://storage.googleapis.com/storage/v1/b?project={}",
            project_id
        ))
        .headers(header_map)
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap());
}
