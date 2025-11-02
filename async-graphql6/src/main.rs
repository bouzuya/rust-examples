fn main() {}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    struct MyObject;

    #[async_graphql::Object]
    impl MyObject {
        async fn int(&self) -> i32 {
            123
        }

        async fn object(&self) -> MyObject {
            MyObject
        }

        #[graphql(complexity = 10)]
        async fn complex(&self) -> i32 {
            456
        }

        #[graphql(complexity = 0)]
        async fn simple(&self) -> i32 {
            0
        }
    }

    #[tokio::test]
    async fn test_analyzer_extension() -> anyhow::Result<()> {
        async fn test_graphql_request(
            request: impl Into<async_graphql::Request>,
            expected_response: &str,
            expected_analyzer_response: &str,
        ) -> anyhow::Result<()> {
            let schema = async_graphql::Schema::build(
                MyObject,
                async_graphql::EmptyMutation,
                async_graphql::EmptySubscription,
            )
            .extension(async_graphql::extensions::Analyzer)
            .finish();
            let request: async_graphql::Request = request.into();
            let mut response: async_graphql::Response = schema.execute(request).await;
            let analyzer_response = response.extensions.remove("analyzer");
            assert_eq!(
                serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
                serde_json::Value::from_str(&expected_response)?
            );
            assert_eq!(
                serde_json::Value::from_str(&serde_json::to_string(&analyzer_response)?)?,
                serde_json::Value::from_str(&expected_analyzer_response)?
            );
            Ok(())
        }

        let request = r#"{ int }"#;
        let expected_response = r#"{"data":{"int":123}}"#;
        let expected_analyzer_response = r#"{"complexity":1,"depth":1}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        let request = r#"{ object { int } }"#;
        let expected_response = r#"{"data":{"object":{"int":123}}}"#;
        let expected_analyzer_response = r#"{"complexity":2,"depth":2}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        let request = r#"{ object { int, object { int } } }"#;
        let expected_response = r#"{"data":{"object":{"int":123,"object":{"int":123}}}}"#;
        let expected_analyzer_response = r#"{"complexity":4,"depth":3}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        // #[graphql(complexity = ...)] を使用した例
        let request = r#"{ complex }"#;
        let expected_response = r#"{"data":{"complex":456}}"#;
        let expected_analyzer_response = r#"{"complexity":10,"depth":1}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        let request = r#"{ simple }"#;
        let expected_response = r#"{"data":{"simple":0}}"#;
        let expected_analyzer_response = r#"{"complexity":0,"depth":1}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        let request = r#"{ object { complex } }"#;
        let expected_response = r#"{"data":{"object":{"complex":456}}}"#;
        let expected_analyzer_response = r#"{"complexity":11,"depth":2}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        let request = r#"{ object { simple } }"#;
        let expected_response = r#"{"data":{"object":{"simple":0}}}"#;
        let expected_analyzer_response = r#"{"complexity":1,"depth":2}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        let request = r#"{ object { int, object { int, complex } } }"#;
        let expected_response =
            r#"{"data":{"object":{"int":123,"object":{"int":123,"complex":456}}}}"#;
        let expected_analyzer_response = r#"{"complexity":14,"depth":3}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        let request = r#"{ object { int, object { int, simple } } }"#;
        let expected_response =
            r#"{"data":{"object":{"int":123,"object":{"int":123,"simple":0}}}}"#;
        let expected_analyzer_response = r#"{"complexity":4,"depth":3}"#;
        test_graphql_request(request, expected_response, expected_analyzer_response).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_my_extension() -> anyhow::Result<()> {
        struct MyExtensionFactory;

        impl async_graphql::extensions::ExtensionFactory for MyExtensionFactory {
            fn create(&self) -> std::sync::Arc<dyn async_graphql::extensions::Extension> {
                std::sync::Arc::new(MyExtension)
            }
        }

        struct MyExtension;

        #[async_trait::async_trait]
        impl async_graphql::extensions::Extension for MyExtension {
            async fn validation(
                &self,
                ctx: &async_graphql::extensions::ExtensionContext<'_>,
                next: async_graphql::extensions::NextValidation<'_>,
            ) -> Result<async_graphql::ValidationResult, Vec<async_graphql::ServerError>>
            {
                let validation_result: async_graphql::ValidationResult = next.run(ctx).await?;

                // 任意の処理 (ここでは標準出力に書き込んでいる)
                println!(
                    "complexity = {}, depth = {}",
                    validation_result.complexity, validation_result.depth
                );

                Ok(validation_result)
            }
        }

        async fn test_graphql_request(
            request: impl Into<async_graphql::Request>,
            expected_response: &str,
        ) -> anyhow::Result<()> {
            let schema = async_graphql::Schema::build(
                MyObject,
                async_graphql::EmptyMutation,
                async_graphql::EmptySubscription,
            )
            .extension(MyExtensionFactory)
            .finish();
            let request: async_graphql::Request = request.into();
            let response: async_graphql::Response = schema.execute(request).await;
            assert_eq!(
                serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
                serde_json::Value::from_str(&expected_response)?
            );
            Ok(())
        }

        let request = r#"{ int }"#;
        let expected_response = r#"{"data":{"int":123}}"#;
        test_graphql_request(request, expected_response).await?;

        Ok(())
    }
}
