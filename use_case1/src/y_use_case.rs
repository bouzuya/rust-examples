use async_trait::async_trait;

use crate::use_case::UseCase;

#[derive(Clone, Debug)]
pub struct YUseCaseInput;

#[derive(Debug, thiserror::Error)]
#[error("YUseCaseError")]
pub struct YUseCaseError;

pub trait YUseCase {}

impl<T> YUseCase for T {}

#[async_trait]
impl<T: YUseCase + Sync> UseCase<YUseCaseInput> for T {
    type O = String;
    type E = YUseCaseError;

    async fn handle(&self, _: YUseCaseInput) -> Result<Self::O, Self::E> {
        Ok("YUseCase".to_string())
    }
}

pub trait HasYUseCase {
    type YUseCase: YUseCase;
    fn y_use_case(&self) -> &Self::YUseCase;
}
