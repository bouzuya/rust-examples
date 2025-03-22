use crate::{
    DateTimeCreated,
    i_calendar::component_properties::{
        Categories, Classification, DateTimeEnd, DateTimeStamp, DateTimeStart, Summary,
        UniqueIdentifier,
    },
};

#[derive(Debug, thiserror::Error)]
#[error("event")]
pub struct EventError(#[from] ErrorInner);

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
    #[error("date-time stamp required")]
    DateTimeStampRequired,
    #[error("date-time start required")]
    DateTimeStartRequired,
    #[error("unique identifier required")]
    UniqueIdentifierRequired,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1>
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    dtstamp: DateTimeStamp,
    uid: UniqueIdentifier,
    dtstart: DateTimeStart,
    class: Option<Classification>,
    created: Option<DateTimeCreated>,
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
            created: builder.created,
            summary: builder.summary,
            dtend: builder.dtend,
            categories: builder.categories,
        })
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        let mut lines = vec![];
        lines.push("BEGIN:VEVENT\r\n".to_owned());
        lines.push(self.uid.into_string());
        lines.push(self.dtstamp.into_string());
        lines.push(self.dtstart.into_string());
        if let Some(dtend) = self.dtend {
            lines.push(dtend.into_string());
        }
        if let Some(summary) = self.summary {
            lines.push(summary.into_string());
        }
        if let Some(class) = self.class {
            lines.push(class.into_string());
        }
        if let Some(created) = self.created {
            lines.push(created.to_escaped());
        }
        for categories in self.categories {
            lines.push(categories.into_string());
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
    created: Option<DateTimeCreated>,
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
            created: None,
            summary: None,
            dtend: None,
            categories: Vec::new(),
        }
    }

    pub fn add_categories(mut self, categories: Categories) -> Self {
        self.categories.push(categories);
        self
    }

    pub fn build(self) -> Result<Event, EventError> {
        Event::from_builder(self)
    }

    pub fn class(mut self, class: Classification) -> Self {
        self.class = Some(class);
        self
    }

    pub fn created(mut self, created: DateTimeCreated) -> Self {
        self.created = Some(created);
        self
    }

    pub fn dtend(mut self, dtend: DateTimeEnd) -> Self {
        self.dtend = Some(dtend);
        self
    }

    pub fn dtstamp(mut self, dtstamp: DateTimeStamp) -> Self {
        self.dtstamp = Some(dtstamp);
        self
    }

    pub fn dtstart(mut self, dtstart: DateTimeStart) -> Self {
        self.dtstart = Some(dtstart);
        self
    }

    pub fn summary(mut self, summary: Summary) -> Self {
        self.summary = Some(summary);
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
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<Event>();

        assert_eq!(
            Event::builder()
                .uid(UniqueIdentifier::from_value(
                    "19970901T130000Z-123401@example.com"
                )?)
                .dtstamp(DateTimeStamp::from_value("19970901T130000Z")?)
                .dtstart(DateTimeStart::from_value("19970903T163000Z")?)
                .dtend(DateTimeEnd::from_value("19970903T190000Z")?)
                .summary(Summary::from_value("Annual Employee Review")?)
                .class(Classification::from_value("PRIVATE")?)
                .add_categories(Categories::from_value("BUSINESS,HUMAN RESOURCES")?)
                .build()?
                .into_string(),
            [
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
            .join("")
        );

        Ok(())
    }
}
