fn auth(context: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
    let parent = context
        .path_node
        .and_then(|it| it.parent.map(|it| it.field_name()));
    let name = context.field().name();

    let data = context.data_unchecked::<Data>();
    let role = data.user_role;

    println!(
        "auth guard called: parent={:?}, name={}, role={}",
        parent, name, role
    );

    let authorized = match (parent, name, role) {
        (None, "hello", Role::Admin) => true,
        (None, "hello", Role::User) => true,
        (None, "parent1", Role::Admin) => true,
        (None, "parent1", Role::User) => true,
        (Some("parent1"), "child1", Role::Admin) => true,
        (Some("parent1"), "child1", Role::User) => false,
        (_, _, _) => false,
    };

    if !authorized {
        return Err(async_graphql::Error::new("Unauthorized"));
    }

    Ok(())
}

struct Query;

#[async_graphql::Object]
impl Query {
    #[graphql(guard = "auth")]
    async fn hello(&self, input: HelloInput) -> HelloOutput {
        HelloOutput {
            message: format!("Hello, {}!", input.name),
        }
    }

    #[graphql(guard = "auth")]
    async fn parent1(&self) -> Parent1Output {
        Parent1Output
    }
}

#[derive(async_graphql::InputObject)]
struct HelloInput {
    name: String,
}

#[derive(async_graphql::SimpleObject)]
struct HelloOutput {
    message: String,
}

struct Parent1Output;

#[async_graphql::Object]
impl Parent1Output {
    #[graphql(guard = "auth")]
    async fn child1(&self) -> String {
        "child1".to_owned()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Role {
    #[allow(dead_code)]
    Admin,
    #[allow(dead_code)]
    User,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
        }
        .fmt(f)
    }
}

struct Data {
    #[allow(dead_code)]
    user_id: String,
    user_role: Role,
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[tokio::test]
    async fn test_hello() {
        let schema = async_graphql::Schema::build(
            Query,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .data(Data {
            user_id: "user123".to_owned(),
            user_role: Role::User,
        })
        .finish();

        let response = schema
            .execute("{ hello(input: { name: \"World\" }) { message } }")
            .await;
        assert_eq!(
            response.data.to_string(),
            r#"{hello: {message: "Hello, World!"}}"#
        );
    }

    #[tokio::test]
    async fn test_parent1() -> anyhow::Result<()> {
        struct MyExtension;

        #[async_trait::async_trait]
        impl async_graphql::extensions::Extension for MyExtension {
            async fn request(
                &self,
                ctx: &async_graphql::extensions::ExtensionContext<'_>,
                next: async_graphql::extensions::NextRequest<'_>,
            ) -> async_graphql::Response {
                println!("request");
                next.run(ctx).await
            }

            async fn resolve(
                &self,
                context: &async_graphql::extensions::ExtensionContext<'_>,
                info: async_graphql::extensions::ResolveInfo<'_>,
                next: async_graphql::extensions::NextResolve<'_>,
            ) -> async_graphql::ServerResult<Option<async_graphql::Value>> {
                println!("resolve");

                // println!("  info.alias {:?}", info.alias);
                // println!("  info.field {:?}", info.field);
                // println!("  info.is_for_introspection {}", info.is_for_introspection);
                // println!("  info.name {}", info.name);
                // println!("  info.parent_type {}", info.parent_type);
                // println!("  info.path_node {:?}", info.path_node);
                // println!("  info.return_type {}", info.return_type);
                //   info.alias None
                //   info.field Field { alias: None, name: Positioned { pos: Pos(1:13), node: Name("child1") }, arguments: [], directives: [], selection_set: Positioned { pos: Pos(0:0), node: SelectionSet { items: [] } } }
                //   info.is_for_introspection false
                //   info.name child1
                //   info.parent_type Parent1Output
                //   info.path_node QueryPathNode { parent: Some(QueryPathNode { parent: None, segment: Name("parent1") }), segment: Name("child1") }
                //   info.return_type String!

                let is_ok = match (info.parent_type, info.name) {
                    ("Query", "parent1") => true,
                    ("Parent1Output", "child1") => {
                        let data = context.data_unchecked::<Data>();
                        match data.user_role {
                            Role::Admin => true,
                            Role::User => false,
                        }
                    }
                    (_, _) => false,
                };
                if is_ok {
                    next.run(context, info).await
                } else {
                    let path = info
                        .path_node
                        .to_string_vec()
                        .into_iter()
                        .map(|it| async_graphql::PathSegment::Field(it))
                        .collect::<Vec<async_graphql::PathSegment>>();
                    Err(async_graphql::ServerError {
                        message: "ExtUnauthorized".to_owned(),
                        source: None,
                        locations: vec![info.field.name.pos],
                        path,
                        extensions: None,
                    })
                }
            }
        }

        struct MyExtensionFactory;

        impl async_graphql::extensions::ExtensionFactory for MyExtensionFactory {
            fn create(&self) -> std::sync::Arc<dyn async_graphql::extensions::Extension> {
                std::sync::Arc::new(MyExtension)
            }
        }

        let schema = async_graphql::Schema::build(
            Query,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .extension(MyExtensionFactory)
        .finish();

        let expected_response = r#"{"data":{"parent1": {"child1": "child1"}}}"#;
        let request: async_graphql::Request = "{ parent1 { child1 } }".into();
        let request = request.data(Data {
            user_id: "user123".to_owned(),
            user_role: Role::Admin,
        });
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(&expected_response)?
        );
        Ok(())
    }
}
