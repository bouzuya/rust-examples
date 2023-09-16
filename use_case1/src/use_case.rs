use async_trait::async_trait;
use tracing::instrument;

#[async_trait]
pub trait UseCase<I: std::fmt::Debug + Send> {
    type O: std::fmt::Debug;
    type E: std::fmt::Debug + std::error::Error;
    #[instrument(skip(self), err, ret)]
    async fn execute(&self, input: I) -> Result<Self::O, Self::E>
    where
        I: 'async_trait,
    {
        self.handle(input).await
    }
    async fn handle(&self, input: I) -> Result<Self::O, Self::E>;
}
