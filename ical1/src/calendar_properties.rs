//! Calendar Properties
//!
//! <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7>
//!

// TODO: <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.2>
// Method

mod calendar_scale;
mod product_identifier;
mod version;

pub use self::calendar_scale::CalendarScale;
pub use self::calendar_scale::CalendarScaleError;
pub use self::product_identifier::ProductIdentifier;
pub use self::product_identifier::ProductIdentifierError;
pub use self::version::Version;
pub use self::version::VersionError;
