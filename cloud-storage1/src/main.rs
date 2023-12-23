fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use cloud_storage::{Client, NewBucket};

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let bucket_name = "bouzuya-cloud-storage-test";

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

        // <https://cloud.google.com/storage/docs/json_api/v1/buckets/delete>
        client.bucket().delete(created_bucket).await?;

        Ok(())
    }
}
