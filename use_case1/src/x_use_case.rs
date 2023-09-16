use async_trait::async_trait;

use crate::{has_prefix::HasPrefix, use_case::UseCase};

#[derive(Debug)]
pub struct XUseCaseInput {
    pub name: String,
}
#[derive(Debug, thiserror::Error)]
#[error("XUseCaseError")]
pub struct XUseCaseError;

pub trait XUseCase: HasPrefix {}

impl<T: HasPrefix> XUseCase for T {}

#[async_trait]
impl<T: XUseCase + Sync> UseCase<XUseCaseInput> for T {
    type O = String;
    type E = XUseCaseError;

    async fn handle(&self, XUseCaseInput { name }: XUseCaseInput) -> Result<Self::O, Self::E> {
        Ok(format!("{}{}", self.prefix(), name))
    }
}

pub trait HasXUseCase {
    type XUseCase: XUseCase;
    fn x_use_case(&self) -> &Self::XUseCase;
}
