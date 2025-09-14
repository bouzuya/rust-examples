use std::{
    cell::LazyCell,
    collections::{BTreeMap, BTreeSet},
};

type UserId = &'static str;
type Permission = &'static str;

// let permissions =
//     BTreeSet::<Permission>::from_iter(["Query::parent1", "Parent1Output::child1"]);
const ROLE_TO_PERMISSIONS: LazyCell<BTreeMap<Role, BTreeSet<Permission>>> = LazyCell::new(|| {
    BTreeMap::<Role, BTreeSet<Permission>>::from_iter([
        (
            Role::Admin,
            BTreeSet::<Permission>::from_iter([
                "Query::hello",
                "Query::parent1",
                "Query::parent2",
                "Parent1Output::child1",
                "Parent2Output::child2",
            ]),
        ),
        (
            Role::User,
            BTreeSet::<Permission>::from_iter(["Query::hello", "Query::parent1"]),
        ),
    ])
});
const USER_TO_ROLES: LazyCell<BTreeMap<UserId, BTreeSet<Role>>> = LazyCell::new(|| {
    BTreeMap::<UserId, BTreeSet<Role>>::from_iter([
        ("admin123", BTreeSet::from_iter([Role::Admin])),
        ("user123", BTreeSet::from_iter([Role::User])),
    ])
});
enum Error {
    RoleNotFound,
    PermissionNotFound,
}

fn user_to_permissions2(user_id: &str) -> Result<BTreeSet<Permission>, Error> {
    let roles = match USER_TO_ROLES.get(user_id) {
        None => return Err(Error::RoleNotFound),
        Some(roles) => roles.clone(),
    };

    let mut permissions = BTreeSet::<Permission>::new();
    for role in roles {
        match ROLE_TO_PERMISSIONS.get(&role) {
            None => return Err(Error::PermissionNotFound),
            Some(ps) => {
                for p in ps {
                    permissions.insert(*p);
                }
            }
        }
    }

    Ok(permissions)
}

fn auth(context: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
    let parent = context
        .path_node
        .and_then(|it| it.parent.map(|it| it.field_name()));
    let name = context.field().name();
    let action = format!(
        "{}::{}",
        match parent {
            None => "Query",
            Some(parent) => match parent {
                "parent1" => "Parent1Output",
                "parent2" => "Parent2Output",
                _ => unreachable!(),
            },
        },
        name
    );

    let data = context.data_unchecked::<Data>();

    println!("auth guard called: parent={:?}, name={}", parent, name);

    let permissions = match user_to_permissions2(data.user_id.as_str()) {
        Ok(permissions) => permissions,
        Err(_) => todo!(),
    };

    // let authorized = match (parent, name, role) {
    //     (None, "hello", Role::Admin) => true,
    //     (None, "hello", Role::User) => true,
    //     (None, "parent1", Role::Admin) => true,
    //     (None, "parent1", Role::User) => true,
    //     (Some("parent1"), "child1", Role::Admin) => true,
    //     (Some("parent1"), "child1", Role::User) => false,
    //     (_, _, _) => false,
    // };

    if !permissions.contains(action.as_str()) {
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

    async fn parent2(&self) -> Parent2Output {
        Parent2Output
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

struct Parent2Output;

#[async_graphql::Object]
impl Parent2Output {
    async fn child2(&self, _input: Child2Input) -> String {
        "child2".to_owned()
    }
}

#[derive(async_graphql::InputObject)]
struct Child2Input {
    id: String,
}

#[allow(dead_code)]
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
        fn error_from_info(
            info: async_graphql::extensions::ResolveInfo<'_>,
        ) -> async_graphql::ServerError {
            let path = info
                .path_node
                .to_string_vec()
                .into_iter()
                .map(|it| async_graphql::PathSegment::Field(it))
                .collect::<Vec<async_graphql::PathSegment>>();
            async_graphql::ServerError {
                message: "ExtUnauthorized".to_owned(),
                source: None,
                locations: vec![info.field.name.pos],
                path,
                extensions: None,
            }
        }

        println!("resolve");

        // println!("  info.alias {:?}", info.alias);
        println!("  info.field {:?}", info.field);
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

        let data = context.data_unchecked::<Data>();
        let user_id = data.user_id.as_str();

        let permissions = match user_to_permissions2(user_id) {
            Ok(permissions) => permissions,
            Err(_) => return Err(error_from_info(info)),
        };

        let action = format!("{}::{}", info.parent_type, info.name);
        if permissions.contains(action.as_str()) {
            next.run(context, info).await
        } else {
            Err(error_from_info(info))
        }
    }
}

#[allow(dead_code)]
struct MyExtensionFactory;

impl async_graphql::extensions::ExtensionFactory for MyExtensionFactory {
    fn create(&self) -> std::sync::Arc<dyn async_graphql::extensions::Extension> {
        std::sync::Arc::new(MyExtension)
    }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    user_id: String,
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
            user_id: "admin123".to_owned(),
        });
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(&expected_response)?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_parent2() -> anyhow::Result<()> {
        let schema = async_graphql::Schema::build(
            Query,
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .extension(MyExtensionFactory)
        .finish();

        let request: async_graphql::Request =
            r#"{ parent2 { child2(input: { id: "abc" }) } }"#.into();
        let expected_response = r#"{"data":{"parent2": {"child2": "child2"}}}"#;
        let request = request.data(Data {
            user_id: "admin123".to_owned(),
        });
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(&expected_response)?
        );
        Ok(())
    }
}
