mod extension;
mod rbac;
mod schema;

struct MyData {
    user: String,
}

struct MyRbac;

#[extension::async_trait]
impl extension::Rbac for MyRbac {
    type Permission = rbac::Permission;
    type Role = rbac::Role;
    type User = rbac::User;

    async fn get_permissions(
        &self,
        _context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
        role: &Self::Role,
    ) -> async_graphql::ServerResult<std::collections::BTreeSet<Self::Permission>> {
        match crate::rbac::ROLE_TO_PERMISSIONS.get(&role).cloned() {
            None => Err(extension::error_from_info(
                "role permissions not found",
                info,
            )),
            Some(permissions) => Ok(permissions),
        }
    }

    async fn get_required_permission(
        &self,
        _context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
    ) -> async_graphql::ServerResult<Self::Permission> {
        let operation = format!("{}::{}", info.parent_type, info.name);
        match <Self::Permission as std::str::FromStr>::from_str(operation.as_str()) {
            Err(_) => Err(extension::error_from_info("unknown permission", info)),
            Ok(required_permission) => Ok(required_permission),
        }
    }

    async fn get_roles(
        &self,
        _context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
        user: &Self::User,
    ) -> async_graphql::ServerResult<std::collections::BTreeSet<Self::Role>> {
        match crate::rbac::USER_TO_ROLES.get(user).cloned() {
            None => Err(extension::error_from_info("user roles not found", info)),
            Some(roles) => Ok(roles),
        }
    }

    async fn get_user(
        &self,
        context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
    ) -> async_graphql::ServerResult<Self::User> {
        let data = context.data_unchecked::<MyData>();
        match <Self::User as std::str::FromStr>::from_str(data.user.as_str()) {
            Err(_) => Err(extension::error_from_info("user not found", info)),
            Ok(user) => Ok(user),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use super::*;

    #[tokio::test]
    async fn test_extension() -> anyhow::Result<()> {
        let request: async_graphql::Request =
            r#"{ parent2 { child2(input: { id: "abc" }) } }"#.into();
        let expected_response = r#"{"data":{"parent2": {"child2": "child2"}}}"#;
        let request = request.data(MyData {
            user: "admin123".to_owned(),
        });
        test_graphql_request(request, expected_response).await?;

        let request: async_graphql::Request =
            r#"{ parent2 { child2(input: { id: "abc" }) } }"#.into();
        let expected_response = &serde_json::json!({
            "data": null,
            "errors": [
                {
                    "locations": [
                        {
                            "column": 3,
                            "line": 1
                        }
                    ],
                    "message": "user roles not found",
                    "path": ["parent2"]
                }
            ]
        })
        .to_string();
        let request = request.data(MyData {
            user: "unknown_user".to_owned(),
        });
        test_graphql_request(request, expected_response).await?;

        let request: async_graphql::Request =
            r#"{ parent2 { child2(input: { id: "abc" }) } }"#.into();
        let expected_response = &serde_json::json!({
            "data": null,
            "errors": [
                {
                    "locations": [
                        {
                            "column": 3,
                            "line": 1
                        }
                    ],
                    "message": "required permission not granted",
                    "path": ["parent2"]
                }
            ]
        })
        .to_string();
        let request = request.data(MyData {
            user: "user123".to_owned(),
        });
        test_graphql_request(request, expected_response).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_parent1() -> anyhow::Result<()> {
        let request: async_graphql::Request = "{ parent1 { child1 } }".into();
        let request = request.data(MyData {
            user: "admin123".to_owned(),
        });
        let expected_response = r#"{"data":{"parent1": {"child1": "child1"}}}"#;
        test_graphql_request(request, expected_response).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_parent2() -> anyhow::Result<()> {
        let request: async_graphql::Request =
            r#"{ parent2 { child2(input: { id: "abc" }) } }"#.into();
        let expected_response = r#"{"data":{"parent2": {"child2": "child2"}}}"#;
        let request = request.data(MyData {
            user: "admin123".to_owned(),
        });
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
        .extension(extension::RbacExtensionFactory::new(MyRbac))
        .finish();
        let request: async_graphql::Request = request.into();
        let response = schema.execute(request).await;
        assert_eq!(
            serde_json::Value::from_str(&serde_json::to_string(&response)?)?,
            serde_json::Value::from_str(&expected_response)?
        );
        Ok(())
    }
}
