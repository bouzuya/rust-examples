struct Query;

#[async_graphql::Object]
impl Query {
    async fn context1(&self, context: &async_graphql::Context<'_>) -> Result<String, String> {
        // context.add_error(error);
        // context.append_http_header(name, value)
        // context.data()
        // context.data_opt()
        // context.data_unchecked()
        let field: async_graphql::SelectionField<'_> = context.field();
        assert_eq!(field.alias(), None);
        assert_eq!(field.arguments(), Ok(vec![]));
        assert_eq!(format!("{:?}", field.directives()), "Ok([])");
        assert_eq!(field.name(), "context1");
        assert_eq!(
            field
                .selection_set()
                .map(|it| it.name().to_owned())
                .collect::<Vec<String>>(),
            Vec::<String>::new()
        );
        // context.http_header_contains(key)
        // context.insert_http_header(name, value)
        // context.look_ahead()
        assert_eq!(
            format!("{:?}", context.path_node),
            "Some(QueryPathNode { parent: None, segment: Name(\"context1\") })"
        );
        Ok("OK".to_owned())
    }

    async fn context2(
        &self,
        context: &async_graphql::Context<'_>,
        input: String,
    ) -> Result<Context2Output, String> {
        assert_eq!(input, "input1");
        let field: async_graphql::SelectionField<'_> = context.field();
        assert_eq!(field.alias(), None);
        assert_eq!(
            field.arguments(),
            Ok(vec![(
                async_graphql::Name::new("input"),
                async_graphql::Value::from_json(serde_json::json!("input1")).unwrap()
            )])
        );
        assert_eq!(format!("{:?}", field.directives()), "Ok([])");
        assert_eq!(field.name(), "context2");
        // assert_eq!(
        //     field
        //         .selection_set()
        //         .map(|it| it.name().to_owned())
        //         .collect::<Vec<String>>(),
        //     vec!["output1".to_owned()] //     vec!["context3".to_owned()]
        // );
        // context.http_header_contains(key)
        // context.insert_http_header(name, value)
        // context.look_ahead()
        assert_eq!(
            format!("{:?}", context.path_node),
            "Some(QueryPathNode { parent: None, segment: Name(\"context2\") })"
        );
        Ok(Context2Output)
    }

    async fn context4(&self, context: &async_graphql::Context<'_>) -> Vec<Context4OutputItem> {
        let field: async_graphql::SelectionField<'_> = context.field();
        assert_eq!(field.alias(), None);
        assert_eq!(field.arguments(), Ok(vec![]));
        assert_eq!(format!("{:?}", field.directives()), "Ok([])");
        assert_eq!(field.name(), "context4");
        assert_eq!(
            field
                .selection_set()
                .map(|it| it.name().to_owned())
                .collect::<Vec<String>>(),
            vec!["context5".to_owned()]
        );
        // context.http_header_contains(key)
        // context.insert_http_header(name, value)
        // context.look_ahead()
        assert_eq!(
            format!("{:?}", context.path_node),
            "Some(QueryPathNode { parent: None, segment: Name(\"context4\") })"
        );
        vec![Context4OutputItem, Context4OutputItem]
    }

    async fn error(&self) -> Result<String, String> {
        Err("An error occurred".to_owned())
    }

    async fn error_panic(&self) -> &str {
        // unreachable_expression になって async_graphql::Object が動作しないため 1 == 1 で回避している
        if 1 == 1 {
            panic!("This is a panic")
        } else {
            "unreachable"
        }
    }

    async fn hello(&self) -> &str {
        "Hello, GraphQL!"
    }
}

struct Context2Output;

#[async_graphql::Object]
impl Context2Output {
    async fn context3(&self, context: &async_graphql::Context<'_>) -> &str {
        let field: async_graphql::SelectionField<'_> = context.field();
        assert_eq!(field.alias(), None);
        assert_eq!(field.arguments(), Ok(vec![]));
        assert_eq!(format!("{:?}", field.directives()), "Ok([])");
        assert_eq!(field.name(), "context3");
        assert_eq!(
            field
                .selection_set()
                .map(|it| it.name().to_owned())
                .collect::<Vec<String>>(),
            Vec::<String>::new()
        );
        // context.http_header_contains(key)
        // context.insert_http_header(name, value)
        // context.look_ahead()
        let path_node: Option<async_graphql::QueryPathNode<'_>> = context.path_node;
        assert_eq!(
            format!("{:?}", path_node),
            "Some(QueryPathNode { parent: Some(QueryPathNode { parent: None, segment: Name(\"context2\") }), segment: Name(\"context3\") })"
        );
        let parent: Option<&async_graphql::QueryPathNode<'_>> = path_node.and_then(|it| it.parent);
        assert_eq!(
            format!("{:?}", parent),
            "Some(QueryPathNode { parent: None, segment: Name(\"context2\") })"
        );
        let segment: Option<async_graphql::QueryPathSegment<'_>> = path_node.map(|it| it.segment);
        assert_eq!(format!("{:?}", segment), "Some(Name(\"context3\"))");
        let name = segment.map(|it| match it {
            async_graphql::QueryPathSegment::Index(i) => format!("index[{i}]"),
            async_graphql::QueryPathSegment::Name(n) => format!("name:{n}"),
        });
        assert_eq!(name, Some("name:context3".to_owned()));
        "context3"
    }

    async fn output1(&self) -> &str {
        "output1"
    }
}

struct Context4OutputItem;

#[async_graphql::Object]
impl Context4OutputItem {
    async fn context5(&self, context: &async_graphql::Context<'_>) -> &str {
        let field: async_graphql::SelectionField<'_> = context.field();
        assert_eq!(field.alias(), None);
        assert_eq!(field.arguments(), Ok(vec![]));
        assert_eq!(format!("{:?}", field.directives()), "Ok([])");
        assert_eq!(field.name(), "context5");
        assert_eq!(
            field
                .selection_set()
                .map(|it| it.name().to_owned())
                .collect::<Vec<String>>(),
            Vec::<String>::new()
        );
        // context.http_header_contains(key)
        // context.insert_http_header(name, value)
        // context.look_ahead()
        let path_node: Option<async_graphql::QueryPathNode<'_>> = context.path_node;
        // assert_eq!(
        //     format!("{:?}", path_node),
        //     "Some(QueryPathNode { parent: Some(QueryPathNode { parent: Some(QueryPathNode { parent: None, segment: Name(\"context4\") }), segment: Index(0) }), segment: Name(\"context5\") })"
        //     or
        //     "Some(QueryPathNode { parent: Some(QueryPathNode { parent: Some(QueryPathNode { parent: None, segment: Name(\"context4\") }), segment: Index(1) }), segment: Name(\"context5\") })"
        // );
        let _parent: Option<&async_graphql::QueryPathNode<'_>> = path_node.and_then(|it| it.parent);
        // assert_eq!(
        //     format!("{:?}", parent),
        //     "Some(QueryPathNode { parent: Some(QueryPathNode { parent: None, segment: Name(\"context4\") }), segment: Index(0) })"
        //     or
        //     "Some(QueryPathNode { parent: Some(QueryPathNode { parent: None, segment: Name(\"context4\") }), segment: Index(1) })"
        // );
        let segment: Option<async_graphql::QueryPathSegment<'_>> = path_node.map(|it| it.segment);
        assert_eq!(format!("{:?}", segment), "Some(Name(\"context5\"))");
        let name = segment.map(|it| match it {
            async_graphql::QueryPathSegment::Index(i) => format!("index[{i}]"),
            async_graphql::QueryPathSegment::Name(n) => format!("name:{n}"),
        });
        assert_eq!(name, Some("name:context5".to_owned()));
        "context5"
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[tokio::test]
    async fn test_hello() -> anyhow::Result<()> {
        let schema = build_schema();
        let request = "{ hello }";
        let expected_response_data = r#"{"data":{"hello":"Hello, GraphQL!"}}"#;
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(expected_response_data)?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_error() -> anyhow::Result<()> {
        let schema = build_schema();
        let request = "{ error }";
        let expected_response_data = r#"{"data":null,"errors":[{"message":"An error occurred","locations":[{"line":1,"column":3}],"path":["error"]}]}"#;
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(expected_response_data)?
        );
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "This is a panic")]
    async fn test_error_panic() {
        let schema = build_schema();
        let request = "{ errorPanic }";
        schema.execute(request).await;
    }

    #[tokio::test]
    async fn test_context1() -> anyhow::Result<()> {
        let schema = build_schema();
        let request = "{ context1 }";
        let expected_response_data = r#"{"data":{"context1":"OK"}}"#;
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(expected_response_data)?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_context2() -> anyhow::Result<()> {
        let schema = build_schema();
        let request = r#"{ context2(input: "input1") { output1 } }"#;
        let expected_response_data = r#"{"data":{"context2":{"output1":"output1"}}}"#;
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(expected_response_data)?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_context3() -> anyhow::Result<()> {
        let schema = build_schema();
        let request = r#"{ context2(input: "input1") { context3 } }"#;
        let expected_response_data = r#"{"data":{"context2":{"context3":"context3"}}}"#;
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(expected_response_data)?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_context4() -> anyhow::Result<()> {
        let schema = build_schema();
        let request = r#"{ context4 { context5 } }"#;
        let expected_response_data =
            r#"{"data":{"context4":[{"context5":"context5"},{"context5":"context5"}]}}"#;
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(expected_response_data)?
        );
        Ok(())
    }

    fn build_schema()
    -> async_graphql::Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>
    {
        async_graphql::Schema::build(
            Query,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .finish()
    }
}
