use crate::google_auth_client::GoogleAuthClient;

mod google_auth_client;

async fn f1() -> google_cloud_auth::Result<()> {
    let credential_config = google_cloud_auth::CredentialConfig::builder()
        .scopes(vec![
            "https://www.googleapis.com/auth/cloud-platform".to_string()
        ])
        .build()?;
    let credential = google_cloud_auth::Credential::find_default(credential_config).await?;
    let access_token = credential.access_token().await?;
    println!("{}", access_token.value);
    Ok(())
}

async fn f2() -> anyhow::Result<()> {
    let credential_config = google_cloud_auth::CredentialConfig::builder()
        .scopes(vec![
            "https://www.googleapis.com/auth/cloud-platform".to_string()
        ])
        .build()?;
    let credential = google_cloud_auth::Credential::find_default(credential_config).await?;
    let client = GoogleAuthClient::new(Some(credential));
    let response = client
        .send(
            http::Request::builder()
                .method(http::Method::GET)
                .uri("https://bouzuya.net/")
                .body(String::default())?,
        )
        .await?;
    println!("status: {}", response.status());
    println!("body: {}", response.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    f1().await?;
    f2().await?;
    Ok(())
}

#[test]
fn test_credential_impls() {
    fn assert_fn<T: Clone + Send + Sync>() {}
    assert_fn::<google_cloud_auth::Credential>();
}
