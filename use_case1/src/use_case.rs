use async_trait::async_trait;

#[async_trait]
pub trait UseCase<I: std::fmt::Debug + Send> {
    type O: std::fmt::Debug;
    type E: std::fmt::Debug + std::error::Error;
    async fn execute(&self, input: I) -> Result<Self::O, Self::E>
    where
        I: 'async_trait,
    {
        println!("execute {:?}", input);
        match self.handle(input).await {
            Ok(output) => {
                println!("execute Ok {:?}", output);
                Ok(output)
            }
            Err(err) => {
                println!("execute Err {:?}", err);
                Err(err)
            }
        }
    }
    async fn handle(&self, input: I) -> Result<Self::O, Self::E>;
}
