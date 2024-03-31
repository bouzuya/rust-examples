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
