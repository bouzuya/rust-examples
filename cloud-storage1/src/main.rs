fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cloud_storage::{Client, NewBucket};

    #[ignore]
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let bucket_name = "xxxxx";

        let client = Client::default();

        // <https://cloud.google.com/storage/docs/json_api/v1/buckets/list>
        let buckets = client.bucket().list().await?;
        assert!(buckets.is_empty());

        // <https://cloud.google.com/storage/docs/json_api/v1/buckets/insert>
        let created_bucket = client
            .bucket()
            .create(&NewBucket {
                name: bucket_name.to_string(),
                ..Default::default()
            })
            .await?;

        let buckets = client.bucket().list().await?;
        assert!(!buckets.is_empty());

        {
            let object_name = "folder/filename.txt";

            match client.object().read(bucket_name, object_name).await {
                Ok(_) => unreachable!(),
                Err(e) => match e {
                    cloud_storage::Error::Google(e) => {
                        assert_eq!(e.error.code, 404);
                    }
                    _ => unreachable!(),
                },
            }

            let content = b"Your file is now on google cloud storage!".to_vec();
            let content_type = "plain/text";

            // <https://cloud.google.com/storage/docs/json_api/v1/objects/insert>
            client
                .object()
                .create(bucket_name, content, object_name, content_type)
                .await?;

            let mut object = client.object().read(bucket_name, object_name).await?;
            assert!(object.metadata.is_none());
            // println!("{:#?}", object);

            let uuid = uuid::Uuid::new_v4().to_string();
            object.metadata = Some({
                let mut metadata = HashMap::new();
                metadata.insert("firebaseStorageDownloadTokens".to_string(), uuid);
                metadata
            });
            client.object().update(&object).await?;

            let object = client.object().read(bucket_name, object_name).await?;
            assert!(object.metadata.as_ref().is_some_and(|m| m.len() == 1));
            // println!("{:#?}", object);

            // <https://cloud.google.com/storage/docs/json_api/v1/objects/delete>
            client.object().delete(bucket_name, object_name).await?;
        }

        // <https://cloud.google.com/storage/docs/json_api/v1/buckets/delete>
        client.bucket().delete(created_bucket).await?;

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test2() -> anyhow::Result<()> {
        let bucket_name = "xxxxx.appspot.com";

        let client = Client::default();

        let object_name = "a.txt";

        match client.object().read(bucket_name, object_name).await {
            Ok(_) => unreachable!(),
            Err(e) => match e {
                cloud_storage::Error::Google(e) => {
                    assert_eq!(e.error.code, 404);
                }
                _ => unreachable!(),
            },
        }

        let content = b"Your file is now on google cloud storage!".to_vec();
        let content_type = "plain/text";

        // <https://cloud.google.com/storage/docs/json_api/v1/objects/insert>
        client
            .object()
            .create(bucket_name, content, object_name, content_type)
            .await?;

        let mut object = client.object().read(bucket_name, object_name).await?;
        assert!(object.metadata.is_none());
        // println!("{:#?}", object);

        let uuid = uuid::Uuid::new_v4().to_string();
        object.metadata = Some({
            let mut metadata = HashMap::new();
            metadata.insert("firebaseStorageDownloadTokens".to_string(), uuid);
            metadata
        });
        client.object().update(&object).await?;

        let object = client.object().read(bucket_name, object_name).await?;
        assert!(object.metadata.as_ref().is_some_and(|m| m.len() == 1));
        // println!("{:#?}", object);

        // <https://cloud.google.com/storage/docs/json_api/v1/objects/delete>
        client.object().delete(bucket_name, object_name).await?;
        Ok(())
    }
}
