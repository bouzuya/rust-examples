#[tokio::main]
async fn main() {
    let credentials: google_cloud_auth::credentials::Credentials =
        google_cloud_auth::credentials::Builder::default()
            .build()
            .unwrap();
    let header_map: http::HeaderMap = credentials.headers(http::Extensions::new()).await.unwrap();
    println!("{:#?}", header_map);
}
