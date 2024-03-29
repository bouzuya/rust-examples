use app::App;
use tracing_subscriber::fmt::format::FmtSpan;
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
    tracing_subscriber::fmt::fmt()
        .with_span_events(FmtSpan::ENTER)
        .init();

    let app = App;

    app.x_use_case()
        .execute(XUseCaseInput {
            name: "bouzuya".to_string(),
        })
        .await?;
    app.y_use_case().execute(YUseCaseInput).await?;
    Ok(())
}
