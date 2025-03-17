use crate::i_calendar::{
    CalendarScaleError, CategoriesError, ClassificationError, DateTimeEndError, DateTimeStampError,
    DateTimeStartError, EventError, ICalendarObjectError, ICalendarStreamError, MethodError,
    ProductIdentifierError, SummaryError, TextError, UniqueIdentifierError, VersionError,
};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    CalendarScale(#[from] CalendarScaleError),
    Categories(#[from] CategoriesError),
    Classification(#[from] ClassificationError),
    DateTimeEnd(#[from] DateTimeEndError),
    DateTimeStamp(#[from] DateTimeStampError),
    DateTimeStart(#[from] DateTimeStartError),
    Event(#[from] EventError),
    ICalendarObject(#[from] ICalendarObjectError),
    ICalendarStream(#[from] ICalendarStreamError),
    Method(#[from] MethodError),
    ProductIdentifier(#[from] ProductIdentifierError),
    Summary(#[from] SummaryError),
    Text(#[from] TextError),
    UniqueIdentifier(#[from] UniqueIdentifierError),
    Version(#[from] VersionError),
}
