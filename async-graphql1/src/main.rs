use axum::{extract::State, response::Html};

struct Query;

#[async_graphql::Object]
impl Query {
    async fn hello(&self) -> String {
        "world".to_string()
    }
}

async fn graphql_get() -> Html<String> {
    Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}

async fn graphql_post(
    State(AppState(schema)): State<AppState>,
    request: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    let request = request.into_inner();
    async_graphql_axum::GraphQLResponse::from(schema.execute(request).await)
}

#[derive(Clone)]
struct AppState(
    async_graphql::Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>,
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = axum::Router::new()
        .route(
            "/graphql",
            axum::routing::get(graphql_get).post(graphql_post),
        )
        .with_state(AppState(
            async_graphql::Schema::build(
                Query,
                async_graphql::EmptyMutation,
                async_graphql::EmptySubscription,
            )
            .finish(),
        ));
    let socket_addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
