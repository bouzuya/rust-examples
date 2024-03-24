use bytes::Bytes;
use http_body_util::Empty;
use hyper::Request;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};

async fn f1() -> anyhow::Result<()> {
    // request body is Empty of bytes::Bytes
    let client: Client<_, Empty<Bytes>> = Client::builder(TokioExecutor::new()).build_http();
    let req = Request::builder()
        .method("GET")
        // not https
        .uri("http://blog.bouzuya.net/2024/03/23/")
        // http::Error
        .body(Empty::new())?;
    // hyper_util::client::legacy::Error
    let response = client.request(req).await?;
    assert_eq!(response.status(), hyper::StatusCode::MOVED_PERMANENTLY);
    // http_body::Body::Error
    let incoming = response.into_body();
    let body_as_vec_of_u8 = http_body_util::BodyExt::collect(incoming)
        .await?
        .to_bytes()
        .to_vec();
    // FromUtf8Error
    let body_as_string = String::from_utf8(body_as_vec_of_u8)?;
    println!("{}", body_as_string);
    Ok(())
}

async fn f2() -> anyhow::Result<()> {
    let client: Client<_, String> = Client::builder(TokioExecutor::new()).build_http();
    let req = Request::builder()
        .method("GET")
        // not https
        .uri("http://blog.bouzuya.net/2024/03/23/")
        // http::Error
        .body(String::default())?;
    // hyper_util::client::legacy::Error
    let response = client.request(req).await?;
    assert_eq!(response.status(), hyper::StatusCode::MOVED_PERMANENTLY);
    // http_body::Body::Error
    let incoming = response.into_body();
    let body_as_vec_of_u8 = http_body_util::BodyExt::collect(incoming)
        .await?
        .to_bytes()
        .to_vec();
    // FromUtf8Error
    let body_as_string = String::from_utf8(body_as_vec_of_u8)?;
    println!("{}", body_as_string);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    f1().await?;
    f2().await?;
    Ok(())
}
