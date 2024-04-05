use std::time::SystemTime;

use v4_sign::{BuildHtmlFormDataOptions, BuildSignedUrlOptions};

#[ignore]
#[tokio::test]
async fn test_build_html_form_data() -> anyhow::Result<()> {
    use v4_sign::build_html_form_data;
    use v4_sign::build_signed_url;
    use v4_sign::ServiceAccountCredentials;

    let ServiceAccountCredentials {
        client_email: service_account_client_email,
        private_key: service_account_private_key,
    } = ServiceAccountCredentials::load(std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?)?;
    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "foo";
    let region = std::env::var("REGION")?;

    let form_params = build_html_form_data(BuildHtmlFormDataOptions {
        service_account_client_email: service_account_client_email.clone(),
        service_account_private_key: service_account_private_key.clone(),
        bucket_name: bucket_name.clone(),
        object_name: object_name.to_string(),
        region: region.clone(),
        expiration: 2,
        now: SystemTime::now(),
    })?;
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

    let url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name: object_name.to_string(),
        region,
        expiration: 2,
        http_method: "GET".to_string(),
        now: SystemTime::now(),
    })?;
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
    use v4_sign::build_signed_url;
    use v4_sign::ServiceAccountCredentials;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "a.txt";
    let region = std::env::var("REGION")?;

    let ServiceAccountCredentials {
        client_email: service_account_client_email,
        private_key: service_account_private_key,
    } = ServiceAccountCredentials::load(std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?)?;
    let url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email: service_account_client_email.clone(),
        service_account_private_key: service_account_private_key.clone(),
        bucket_name: bucket_name.clone(),
        object_name: object_name.to_string(),
        region: region.clone(),
        expiration: 2,
        http_method: "POST".to_string(),
        now: SystemTime::now(),
    })?;
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("key", object_name)
        .part(
            "file",
            reqwest::multipart::Part::bytes(include_bytes!("./a.txt").to_vec()),
        );
    let response = client.post(url).multipart(form).send().await?;
    assert_eq!(response.status().as_u16(), 204);

    let url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name: object_name.to_string(),
        region,
        expiration: 2,
        http_method: "GET".to_string(),
        now: SystemTime::now(),
    })?;
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
    use v4_sign::build_signed_url;
    use v4_sign::ServiceAccountCredentials;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "a.txt";
    let region = std::env::var("REGION")?;

    let ServiceAccountCredentials {
        client_email: service_account_client_email,
        private_key: service_account_private_key,
    } = ServiceAccountCredentials::load(std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?)?;
    let signed_url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name: object_name.to_string(),
        region,
        expiration: 2,
        http_method: "GET".to_string(),
        now: SystemTime::now(),
    })?;

    let response = reqwest::get(signed_url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await?, "foo\n");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_get_timeout() -> anyhow::Result<()> {
    use v4_sign::build_signed_url;
    use v4_sign::ServiceAccountCredentials;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "a.txt";
    let region = std::env::var("REGION")?;

    let ServiceAccountCredentials {
        client_email: service_account_client_email,
        private_key: service_account_private_key,
    } = ServiceAccountCredentials::load(std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?)?;
    let signed_url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name: object_name.to_string(),
        region,
        expiration: 1,
        http_method: "GET".to_string(),
        now: SystemTime::now(),
    })?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let response = reqwest::get(signed_url).await?;
    assert_eq!(response.status().as_u16(), 400);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_post_invalid_http_method() -> anyhow::Result<()> {
    use v4_sign::build_signed_url;
    use v4_sign::ServiceAccountCredentials;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "a.txt";
    let region = std::env::var("REGION")?;

    let ServiceAccountCredentials {
        client_email: service_account_client_email,
        private_key: service_account_private_key,
    } = ServiceAccountCredentials::load(std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?)?;
    let signed_url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name: object_name.to_string(),
        region,
        expiration: 2,
        http_method: "POST".to_string(),
        now: SystemTime::now(),
    })?;

    let response = reqwest::get(signed_url).await?;
    assert_eq!(response.status().as_u16(), 403);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_post() -> anyhow::Result<()> {
    use v4_sign::build_signed_url;
    use v4_sign::ServiceAccountCredentials;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "b.txt";
    let region = std::env::var("REGION")?;

    let ServiceAccountCredentials {
        client_email: service_account_client_email,
        private_key: service_account_private_key,
    } = ServiceAccountCredentials::load(std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?)?;
    let url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email: service_account_client_email.clone(),
        service_account_private_key: service_account_private_key.clone(),
        bucket_name: bucket_name.clone(),
        object_name: object_name.to_string(),
        region: region.clone(),
        expiration: 2,
        http_method: "POST".to_string(),
        now: SystemTime::now(),
    })?;
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("key", object_name)
        .text("file", "bar");
    let response = client.post(url).multipart(form).send().await?;
    assert_eq!(response.status().as_u16(), 204);

    let url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name: object_name.to_string(),
        region,
        expiration: 2,
        http_method: "GET".to_string(),
        now: SystemTime::now(),
    })?;
    let response = reqwest::get(url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await?, "bar");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_post_bin() -> anyhow::Result<()> {
    use v4_sign::build_signed_url;
    use v4_sign::ServiceAccountCredentials;

    let bucket_name = std::env::var("BUCKET_NAME")?;
    let object_name = "c.png";
    let region = std::env::var("REGION")?;

    let ServiceAccountCredentials {
        client_email: service_account_client_email,
        private_key: service_account_private_key,
    } = ServiceAccountCredentials::load(std::env::var("GOOGLE_APPLICATION_CREDENTIALS")?)?;
    let url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email: service_account_client_email.clone(),
        service_account_private_key: service_account_private_key.clone(),
        bucket_name: bucket_name.clone(),
        object_name: object_name.to_string(),
        region: region.clone(),
        expiration: 2,
        http_method: "POST".to_string(),
        now: SystemTime::now(),
    })?;
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("key", object_name)
        .part(
            "file",
            reqwest::multipart::Part::bytes(include_bytes!("./c.png").to_vec()),
        );
    let response = client.post(url).multipart(form).send().await?;
    assert_eq!(response.status().as_u16(), 204);

    let url = build_signed_url(BuildSignedUrlOptions {
        service_account_client_email,
        service_account_private_key,
        bucket_name,
        object_name: object_name.to_string(),
        region,
        expiration: 2,
        http_method: "GET".to_string(),
        now: SystemTime::now(),
    })?;
    let response = reqwest::get(url).await?;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.bytes().await?.to_vec(),
        include_bytes!("./c.png").to_vec()
    );

    Ok(())
}
