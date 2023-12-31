use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, ListDocumentsRequest,
};
use google_authz::{Credentials, GoogleAuthz};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Required: `PROJECT_ID` env var
    let project_id = std::env::var("PROJECT_ID")?;
    // Required: `GOOGLE_APPLICATION_CREDENTIALS` env var
    let credentials = Credentials::builder().build().await?;
    let channel = Channel::from_static("https://firestore.googleapis.com")
        .connect()
        .await?;
    let client = GoogleAuthz::builder(channel)
        .credentials(credentials)
        .build()
        .await;
    let mut client = FirestoreClient::new(client);
    let response = client
        .list_documents(ListDocumentsRequest {
            parent: format!("projects/{}/databases/(default)/documents", project_id),
            collection_id: "users".to_string(),
            ..Default::default()
        })
        .await?
        .into_inner();
    // It works
    println!("{:#?}", response);
    Ok(())
}
