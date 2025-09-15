use std::collections::BTreeSet;

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

#[async_trait]
pub trait Rbac {
    type Permission: Ord + Send;
    type Role: Ord + Send;
    type User: Send;

    async fn get_permissions(
        &self,
        context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
        role: &Self::Role,
    ) -> async_graphql::ServerResult<BTreeSet<Self::Permission>>;

    async fn get_required_permission(
        &self,
        context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
    ) -> async_graphql::ServerResult<Self::Permission>;

    async fn get_roles(
        &self,
        context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
        user: &Self::User,
    ) -> async_graphql::ServerResult<BTreeSet<Self::Role>>;

    async fn get_user(
        &self,
        context: &async_graphql::extensions::ExtensionContext<'_>,
        info: &async_graphql::extensions::ResolveInfo<'_>,
    ) -> async_graphql::ServerResult<Self::User>;
}

pub struct RbacExtension<T> {
    inner: T,
}

#[async_trait]
impl<T: Rbac + Send + Sync + 'static> async_graphql::extensions::Extension for RbacExtension<T> {
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
        let user = self.inner.get_user(context, &info).await?;
        let roles = self.inner.get_roles(context, &info, &user).await?;
        let mut permissions = BTreeSet::new();
        for role in roles {
            let ps = self.inner.get_permissions(context, &info, &role).await?;
            permissions.extend(ps);
        }
        let required_permission = self.inner.get_required_permission(context, &info).await?;
        if permissions.contains(&required_permission) {
            next.run(context, info).await
        } else {
            Err(error_from_info("required permission not granted", &info))
        }
    }
}

pub struct RbacExtensionFactory<T> {
    inner: std::sync::Arc<RbacExtension<T>>,
}

impl<T: Rbac + Send + Sync + 'static> RbacExtensionFactory<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: std::sync::Arc::new(RbacExtension::<T> { inner }),
        }
    }
}

impl<T: Rbac + Send + Sync + 'static> async_graphql::extensions::ExtensionFactory
    for RbacExtensionFactory<T>
{
    fn create(&self) -> std::sync::Arc<dyn async_graphql::extensions::Extension> {
        self.inner.clone()
    }
}
