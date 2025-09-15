pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn parent1(&self, _input: Option<Parent1Input>) -> Parent1Output {
        Parent1Output
    }

    async fn parent2(&self, _input: Option<Parent2Input>) -> Parent2Output {
        Parent2Output
    }
}

#[derive(Default, async_graphql::InputObject)]
struct Parent1Input {
    id: Option<String>,
}

struct Parent1Output;

#[async_graphql::Object]
impl Parent1Output {
    async fn child1(&self) -> String {
        "child1".to_owned()
    }
}

#[derive(Default, async_graphql::InputObject)]
struct Parent2Input {
    id: Option<String>,
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
