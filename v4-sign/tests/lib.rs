#[test]
fn test_credential_scope() -> anyhow::Result<()> {
    use v4_sign::CredentialScope;
    use v4_sign::Date;
    use v4_sign::Location;
    use v4_sign::RequestType;
    use v4_sign::Service;

    let credential_scope = CredentialScope::new(
        Date::try_from("20240402")?,
        Location::try_from("us-central1")?,
        Service::Storage,
        RequestType::Goog4Request,
    )?;
    assert_eq!(
        credential_scope.to_string(),
        "20240402/us-central1/storage/goog4_request"
    );
    Ok(())
}

#[test]
fn test_signing_algorithm() -> anyhow::Result<()> {
    use v4_sign::SigningAlgorithm;

    assert_eq!(
        SigningAlgorithm::Goog4RsaSha256.as_ref(),
        "GOOG4-RSA-SHA256"
    );
    assert_eq!(
        SigningAlgorithm::Goog4HmacSha256.as_ref(),
        "GOOG4-HMAC-SHA256"
    );
    assert_eq!(
        SigningAlgorithm::Aws4HmacSha256.as_ref(),
        "AWS4-HMAC-SHA256"
    );
    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_html_form() -> anyhow::Result<()> {
    use v4_sign::html_form_params;
    use v4_sign::signed_url;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "/foo";
    let region = std::env::var("REGION")?;

    let form_params = html_form_params(&bucket_name, object_name, &region, 2)?;
    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://storage.googleapis.com/{}", bucket_name))
        .multipart({
            let mut form = reqwest::multipart::Form::new();
            for (name, value) in form_params {
                form = form.text(name, value)
            }
            form.part(
                "file",
                reqwest::multipart::Part::bytes(include_bytes!("./a.txt").to_vec()),
            )
        })
        .send()
        .await?;
    assert_eq!(response.status().as_u16(), 204);
    assert_eq!(response.text().await?, "");

    let url = signed_url(&bucket_name, object_name, &region, 2, "GET")?;
    let response = reqwest::get(url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.bytes().await?.to_vec(),
        include_bytes!("./a.txt").to_vec()
    );

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_setup_a_txt() -> anyhow::Result<()> {
    use v4_sign::signed_url;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "/a.txt";
    let region = std::env::var("REGION")?;

    let url = signed_url(&bucket_name, object_name, &region, 2, "POST")?;
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("key", object_name)
        .part(
            "file",
            reqwest::multipart::Part::bytes(include_bytes!("./a.txt").to_vec()),
        );
    let response = client.post(url).multipart(form).send().await?;
    assert_eq!(response.status().as_u16(), 204);

    let url = signed_url(&bucket_name, object_name, &region, 2, "GET")?;
    let response = reqwest::get(url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.bytes().await?.to_vec(),
        include_bytes!("./a.txt").to_vec()
    );

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get() -> anyhow::Result<()> {
    use v4_sign::signed_url;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "/a.txt";
    let region = std::env::var("REGION")?;

    let signed_url = signed_url(&bucket_name, object_name, &region, 2, "GET")?;

    let response = reqwest::get(signed_url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await?, "foo\n");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_timeout() -> anyhow::Result<()> {
    use v4_sign::signed_url;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "/a.txt";
    let region = std::env::var("REGION")?;

    let signed_url = signed_url(&bucket_name, object_name, &region, 1, "GET")?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let response = reqwest::get(signed_url).await?;
    assert_eq!(response.status().as_u16(), 400);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_post_invalid_http_method() -> anyhow::Result<()> {
    use v4_sign::signed_url;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "/a.txt";
    let region = std::env::var("REGION")?;

    let signed_url = signed_url(&bucket_name, object_name, &region, 2, "POST")?;

    let response = reqwest::get(signed_url).await?;
    assert_eq!(response.status().as_u16(), 403);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_post() -> anyhow::Result<()> {
    use v4_sign::signed_url;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "/b.txt";
    let region = std::env::var("REGION")?;

    let url = signed_url(&bucket_name, object_name, &region, 2, "POST")?;
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("key", object_name)
        .text("file", "bar");
    let response = client.post(url).multipart(form).send().await?;
    assert_eq!(response.status().as_u16(), 204);

    let url = signed_url(&bucket_name, object_name, &region, 2, "GET")?;
    let response = reqwest::get(url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await?, "bar");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_post_bin() -> anyhow::Result<()> {
    use v4_sign::signed_url;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "/c.png";
    let region = std::env::var("REGION")?;

    let url = signed_url(&bucket_name, object_name, &region, 2, "POST")?;
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("key", object_name)
        .part(
            "file",
            reqwest::multipart::Part::bytes(include_bytes!("./c.png").to_vec()),
        );
    let response = client.post(url).multipart(form).send().await?;
    assert_eq!(response.status().as_u16(), 204);

    let url = signed_url(&bucket_name, object_name, &region, 2, "GET")?;
    let response = reqwest::get(url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.bytes().await?.to_vec(),
        include_bytes!("./c.png").to_vec()
    );

    Ok(())
}
