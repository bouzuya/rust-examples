// re-export
pub use async_graphql;
pub use async_trait::async_trait;

pub fn error_from_info(
    message: impl Into<String>,
    info: &async_graphql::extensions::ResolveInfo<'_>,
) -> async_graphql::ServerError {
    let path = info
        .path_node
        .to_string_vec()
        .into_iter()
        .map(|it| async_graphql::PathSegment::Field(it))
        .collect::<Vec<async_graphql::PathSegment>>();
    async_graphql::ServerError {
        message: message.into(),
        source: None,
        locations: vec![info.field.name.pos],
        path,
        extensions: None,
    }
}

pub struct MyExtension;

#[async_trait]
impl async_graphql::extensions::Extension for MyExtension {
    async fn request(
        &self,
        ctx: &async_graphql::extensions::ExtensionContext<'_>,
        next: async_graphql::extensions::NextRequest<'_>,
    ) -> async_graphql::Response {
        next.run(ctx).await
    }

    async fn resolve(
        &self,
        context: &async_graphql::extensions::ExtensionContext<'_>,
        info: async_graphql::extensions::ResolveInfo<'_>,
        next: async_graphql::extensions::NextResolve<'_>,
    ) -> async_graphql::ServerResult<Option<async_graphql::Value>> {
        match info.parent_type {
            "Parent2Output" => match info.name {
                "child2" => {
                    for (name, value) in &info.field.arguments {
                        assert_eq!(name.node.as_str(), "input");
                        match &value.node {
                            async_graphql_value::Value::Variable(v) => {
                                println!("variable = {}", v);
                                todo!()
                            }
                            async_graphql_value::Value::Object(o) => {
                                let v: &async_graphql_value::Value = o.get("id").unwrap();
                                assert!(
                                    v.to_string() == r#""abc""#
                                        || v.to_string() == r#""def""#
                                        || v.to_string() == r#"$id"#
                                );
                            }
                            _ => unreachable!(),
                        }
                    }
                    return next.run(context, info).await;
                }
                _ => return next.run(context, info).await,
            },
            _ => return next.run(context, info).await,
        }
    }
}

pub struct MyExtensionFactory;

impl async_graphql::extensions::ExtensionFactory for MyExtensionFactory {
    fn create(&self) -> std::sync::Arc<dyn async_graphql::extensions::Extension> {
        std::sync::Arc::new(MyExtension)
    }
}
