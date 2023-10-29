use std::{env, path::Path};

use google_api_proto::google::firestore::v1::{
    firestore_client::FirestoreClient, ListDocumentsRequest,
};
use google_authz::{Credentials, GoogleAuthz};
use tonic::{transport::Channel, Request};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project_id = env::var("PROJECT_ID")?;

    // 指定しなくても読まれるが、明示的に指定している
    let json_path_as_str = env::var("GOOGLE_APPLICATION_CREDENTIALS")?;
    let credentials = Credentials::builder()
        .json_file(Path::new(json_path_as_str.as_str()))
        .build()
        .await?;
    let channel = Channel::from_static("https://firestore.googleapis.com")
        .connect()
        .await?;
    let channel = GoogleAuthz::builder(channel)
        .credentials(credentials)
        .build()
        .await;

    let database_id = "(default)";
    let collection_name = "users";

    let response = FirestoreClient::new(channel)
        .list_documents(Request::new(ListDocumentsRequest {
            parent: format!(
                "projects/{}/databases/{}/documents",
                project_id, database_id
            ),
            collection_id: collection_name.to_owned(),
            page_size: 100,
            ..Default::default()
        }))
        .await?;
    println!("response = {:#?}", response);
    Ok(())
}
