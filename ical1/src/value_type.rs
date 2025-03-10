//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3>
//! Property Value Data Types

mod date_time;
mod text;

pub use self::date_time::DateTime;
pub use self::date_time::DateTimeError;
pub use self::text::Text;
pub use self::text::TextError;
