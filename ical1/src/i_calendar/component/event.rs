use crate::i_calendar::property::{
    Categories, CategoriesError, Classification, ClassificationError, DateTimeEnd,
    DateTimeEndError, DateTimeStamp, DateTimeStampError, DateTimeStart, DateTimeStartError,
    Summary, SummaryError, UniqueIdentifier, UniqueIdentifierError,
};

#[derive(Debug, thiserror::Error)]
#[error("event")]
pub struct EventError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("build")]
    Build(#[source] PrivateEventBuilderError),
    #[error("categories")]
    Categories(#[source] CategoriesError),
    #[error("classification")]
    Classification(#[source] ClassificationError),
    #[error("date-time end")]
    DateTimeEnd(#[source] DateTimeEndError),
    #[error("date-time stamp")]
    DateTimeStamp(#[source] DateTimeStampError),
    #[error("date-time stamp required")]
    DateTimeStampRequired,
    #[error("date-time start")]
    DateTimeStart(#[source] DateTimeStartError),
    #[error("date-time start required")]
    DateTimeStartRequired,
    #[error("invalid format")]
    InvalidFormat,
    #[error("summary")]
    Summary(#[source] SummaryError),
    #[error("unique identifier")]
    UniqueIdentifier(#[source] UniqueIdentifierError),
    #[error("unique identifier required")]
    UniqueIdentifierRequired,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1>
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, derive_builder::Builder)]
#[builder(name = "PrivateEventBuilder")]
pub struct Event {
    dtstamp: DateTimeStamp,
    uid: UniqueIdentifier,
    dtstart: DateTimeStart,
    class: Option<Classification>,
    summary: Option<Summary>,
    dtend: Option<DateTimeEnd>,
    categories: Vec<Categories>,
}

impl Event {
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    fn from_builder(builder: EventBuilder) -> Result<Self, EventError> {
        Ok(Self {
            dtstamp: builder
                .dtstamp
                .ok_or_else(|| ErrorInner::DateTimeStampRequired)?,
            uid: builder
                .uid
                .ok_or_else(|| ErrorInner::UniqueIdentifierRequired)?,
            dtstart: builder
                .dtstart
                .ok_or_else(|| ErrorInner::DateTimeStartRequired)?,
            class: builder.class,
            summary: builder.summary,
            dtend: builder.dtend,
            categories: builder.categories,
        })
    }

    pub(in crate::i_calendar) fn from_string(value: String) -> Result<Self, EventError> {
        if value.starts_with("BEGIN:VEVENT\r\n") && value.ends_with("END:VEVENT\r\n") {
            let mut lines = value
                .trim_start_matches("BEGIN:VEVENT\r\n")
                .trim_end_matches("END:VEVENT\r\n")
                .split("\r\n")
                .collect::<Vec<&str>>();
            lines.pop();
            let mut builder = PrivateEventBuilder::default();
            for line in lines {
                if line.starts_with("UID:") {
                    let uid = UniqueIdentifier::from_string(format!("{}\r\n", line))
                        .map_err(ErrorInner::UniqueIdentifier)?;
                    builder.uid(uid);
                } else if line.starts_with("DTSTAMP:") {
                    let dtstamp = DateTimeStamp::from_string(format!("{}\r\n", line))
                        .map_err(ErrorInner::DateTimeStamp)?;
                    builder.dtstamp(dtstamp);
                } else if line.starts_with("DTSTART:") {
                    let dtstart = DateTimeStart::try_from(format!("{}\r\n", line))
                        .map_err(ErrorInner::DateTimeStart)?;
                    builder.dtstart(dtstart);
                } else if line.starts_with("DTEND:") {
                    let dtend = DateTimeEnd::try_from(format!("{}\r\n", line))
                        .map_err(ErrorInner::DateTimeEnd)?;
                    builder.dtend(Some(dtend));
                } else if line.starts_with("SUMMARY:") {
                    let summary =
                        Summary::try_from(format!("{}\r\n", line)).map_err(ErrorInner::Summary)?;
                    builder.summary(Some(summary));
                } else if line.starts_with("CLASS:") {
                    let class = Classification::try_from(format!("{}\r\n", line))
                        .map_err(ErrorInner::Classification)?;
                    builder.class(Some(class));
                } else if line.starts_with("CATEGORIES:") {
                    let categories = Categories::try_from(format!("{}\r\n", line))
                        .map_err(ErrorInner::Categories)?;
                    builder.categories(
                        builder
                            .categories
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .chain(std::iter::once(categories))
                            .collect::<Vec<Categories>>(),
                    );
                } else {
                    Err(ErrorInner::InvalidFormat)?
                }
            }
            Ok(builder.build().map_err(ErrorInner::Build)?)
        } else {
            Err(ErrorInner::InvalidFormat)?
        }
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        let mut lines = vec![];
        lines.push("BEGIN:VEVENT\r\n".to_owned());
        lines.push(self.uid.into_string());
        lines.push(self.dtstamp.into_string());
        lines.push(String::from(self.dtstart));
        if let Some(dtend) = self.dtend {
            lines.push(String::from(dtend));
        }
        if let Some(summary) = self.summary {
            lines.push(String::from(summary));
        }
        if let Some(class) = self.class {
            lines.push(String::from(class));
        }
        for categories in self.categories {
            lines.push(String::from(categories));
        }
        lines.push("END:VEVENT\r\n".to_owned());
        lines.join("")
    }
}

pub struct EventBuilder {
    dtstamp: Option<DateTimeStamp>,
    uid: Option<UniqueIdentifier>,
    dtstart: Option<DateTimeStart>,
    class: Option<Classification>,
    summary: Option<Summary>,
    dtend: Option<DateTimeEnd>,
    categories: Vec<Categories>,
}

impl EventBuilder {
    fn new() -> Self {
        Self {
            dtstamp: None,
            uid: None,
            dtstart: None,
            class: None,
            summary: None,
            dtend: None,
            categories: Vec::new(),
        }
    }

    pub fn build(self) -> Result<Event, EventError> {
        Event::from_builder(self)
    }

    pub fn dtstamp(mut self, dtstamp: DateTimeStamp) -> Self {
        self.dtstamp = Some(dtstamp);
        self
    }

    pub fn uid(mut self, uid: UniqueIdentifier) -> Self {
        self.uid = Some(uid);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Event>();

        let s = [
            "BEGIN:VEVENT\r\n",
            "UID:19970901T130000Z-123401@example.com\r\n",
            "DTSTAMP:19970901T130000Z\r\n",
            "DTSTART:19970903T163000Z\r\n",
            "DTEND:19970903T190000Z\r\n",
            "SUMMARY:Annual Employee Review\r\n",
            "CLASS:PRIVATE\r\n",
            "CATEGORIES:BUSINESS,HUMAN RESOURCES\r\n",
            "END:VEVENT\r\n",
        ]
        .join("");
        assert_eq!(Event::from_string(s.clone())?.into_string(), s);

        Ok(())
    }
}
