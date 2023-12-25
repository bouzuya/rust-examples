use std::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Poll, RawWaker, RawWakerVTable, Waker},
};

use google_authz::{Credentials, GoogleAuthz};
use reqwest::Client;
use tower_service::Service as _;

struct HttpClient(reqwest::Client);

impl tower_service::Service<http::Request<Vec<u8>>> for HttpClient {
    type Response = reqwest::Response;

    type Error = reqwest::Error;

    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<Vec<u8>>) -> Self::Future {
        let req =
            reqwest::Request::try_from(req).expect("reqwest::Request::try_from(http::Request)");
        Box::pin(self.0.execute(req))
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn test() -> anyhow::Result<()> {
    let credentials = Credentials::builder().no_credentials().build().await?;
    let service = HttpClient(Client::new());
    let mut service = GoogleAuthz::builder(service)
        .credentials(credentials)
        .build()
        .await;

    let response = service
        .call(
            http::Request::builder()
                .method("GET")
                .uri("https://bouzuya.net")
                .body(Vec::new())?,
        )
        .await?;
    let response_body = response.text().await?;
    assert!(response_body.contains("<title>bouzuya.net</title>"));
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let credentials = Credentials::builder()
        .scopes(&["https://www.googleapis.com/auth/drive.readonly"])
        .build()
        .await?;
    let service = HttpClient(Client::new());
    let mut service = GoogleAuthz::builder(service)
        .credentials(credentials)
        .build()
        .await;

    let waker = unsafe {
        const VTABLE: RawWakerVTable = RawWakerVTable::new(
            // Cloning just returns a new no-op raw waker
            |_| RAW,
            // `wake` does nothing
            |_| {},
            // `wake_by_ref` does nothing
            |_| {},
            // Dropping does nothing as we don't allocate anything
            |_| {},
        );
        const RAW: RawWaker = RawWaker::new(ptr::null(), &VTABLE);
        Waker::from_raw(RAW)
    };

    let mut x = service.poll_ready(&mut std::task::Context::from_waker(&waker));
    loop {
        match x {
            Poll::Ready(Ok(())) => break,
            Poll::Ready(Err(e)) => panic!("{:?}", e),
            Poll::Pending => {}
        }
        x = service.poll_ready(&mut std::task::Context::from_waker(&waker));
    }
    println!("{:?}", x);

    // <https://developers.google.com/drive/api/reference/rest/v3/files/list?hl=ja>
    // <https://developers.google.com/drive/api/reference/rest/v3/files/get?hl=ja>
    let folder_id = "1spieLw_OMRx59B0G1SsMABC5ETKNqkqb";
    let file_id = "1DOeJ6jAlau1EgBpFRDAg1KW8Adz6-GHU";
    println!("folder_id: {}, file_id: {}", folder_id, file_id);
    let response = service
        .call(
            http::Request::builder()
                .method("GET")
                // list
                // .uri("https://www.googleapis.com/drive/v3/files")
                // get (download)
                .uri(format!(
                    "https://www.googleapis.com/drive/v3/files/{}?alt=media",
                    file_id
                ))
                .body(Vec::new())?,
        )
        .await?;
    let response_body = response.text().await?;
    println!("{}", response_body);
    Ok(())
}
