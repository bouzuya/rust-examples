#[tokio::main]
async fn main() {
    let project_id = std::env::var("PROJECT_ID").unwrap();

    let credentials: google_cloud_auth::credentials::Credentials =
        google_cloud_auth::credentials::Builder::default()
            .build()
            .unwrap();
    let header_map: http::HeaderMap = credentials.headers(http::Extensions::new()).await.unwrap();

    let http = false;

    if http {
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
    } else {
        let metadata = tonic::metadata::MetadataMap::from_headers(header_map);
        let mut firestore_client = googleapis_tonic_google_firestore_v1::google::firestore::v1::firestore_client::FirestoreClient::with_interceptor(
            tonic::transport::Channel::from_static("https://firestore.googleapis.com")
                        .tls_config(tonic::transport::ClientTlsConfig::new().with_webpki_roots()).unwrap()
                .connect()
                .await
                .unwrap(),
                |mut request: tonic::Request<()>| -> Result<tonic::Request<()>, tonic::Status> {
                    for kv in metadata.iter() {
                        match kv {
                            tonic::metadata::KeyAndValueRef::Ascii(key, val) => {
                                request.metadata_mut().insert(key, val.clone());
                            }
                            tonic::metadata::KeyAndValueRef::Binary(key, val) => {
                                request.metadata_mut().insert_bin(key, val.clone());
                            },
                        }

                    }
                    Ok(request)
                }
        );
        let list_collection_ids_response = firestore_client
            .list_collection_ids(googleapis_tonic_google_firestore_v1::google::firestore::v1::ListCollectionIdsRequest {
                parent: format!("projects/{}/databases/(default)/documents", project_id),
                page_size: 10,
                page_token: "".to_string(),
                consistency_selector: None,
            })
            .await
            .unwrap()
            .into_inner();
        println!("{:#?}", list_collection_ids_response);
    }
}
