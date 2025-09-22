mod extension;
mod schema;

struct MyData {
    user: String,
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[tokio::test]
    async fn test_extension() -> anyhow::Result<()> {
        let request = r#"{ parent2 { child2(input: { id: "abc" }) } }"#;
        let expected_response = r#"{"data":{"parent2": {"child2": "child2"}}}"#;
        test_graphql_request(request, expected_response).await?;

        let request = r#"{ parent2 { child2(input: { id: "def" }) } }"#;
        let expected_response = r#"{"data":{"parent2": {"child2": "child2"}}}"#;
        test_graphql_request(request, expected_response).await?;

        let request = r#"query myQuery($id: String!) { parent2 { child2(input: { id: $id }) } }"#;
        let expected_response = r#"{"data":{"parent2": {"child2": "child2"}}}"#;
        test_graphql_request(request, expected_response).await?;

        Ok(())
    }

    async fn test_graphql_request(
        request: impl Into<async_graphql::Request>,
        expected_response: &str,
    ) -> anyhow::Result<()> {
        let schema = async_graphql::Schema::build(
            schema::QueryRoot,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .extension(extension::MyExtensionFactory)
        .finish();
        let request: async_graphql::Request = request.into();
        let request = request.variables(async_graphql::Variables::from_json(serde_json::json!({
            "id": "ghi",
        })));
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(&expected_response)?
        );
        Ok(())
    }
}
