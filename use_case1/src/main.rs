use app::App;
use use_case::UseCase;
use x_use_case::{HasXUseCase, XUseCaseInput};
use y_use_case::{HasYUseCase, YUseCaseInput};

mod app;
mod has_prefix;
mod use_case;
mod x_use_case;
mod y_use_case;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App;

    app.x_use_case()
        .execute(XUseCaseInput {
            name: "bouzuya".to_string(),
        })
        .await?;
    app.y_use_case().execute(YUseCaseInput).await?;
    Ok(())
}
