use crate::{has_prefix::HasPrefix, x_use_case::HasXUseCase, y_use_case::HasYUseCase};

pub struct App;

impl HasPrefix for App {
    fn prefix(&self) -> String {
        "Hello, ".to_string()
    }
}

impl HasXUseCase for App {
    type XUseCase = Self;

    fn x_use_case(&self) -> &Self::XUseCase {
        self
    }
}

impl HasYUseCase for App {
    type YUseCase = Self;

    fn y_use_case(&self) -> &Self::YUseCase {
        self
    }
}
