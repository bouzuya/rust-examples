use crate::property::{
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
    Build(#[source] EventBuilderError),
    #[error("categories")]
    Categories(#[source] CategoriesError),
    #[error("classification")]
    Classification(#[source] ClassificationError),
    #[error("date-time end")]
    DateTimeEnd(#[source] DateTimeEndError),
    #[error("date-time stamp")]
    DateTimeStamp(#[source] DateTimeStampError),
    #[error("date-time start")]
    DateTimeStart(#[source] DateTimeStartError),
    #[error("invalid format")]
    InvalidFormat,
    #[error("summary")]
    Summary(#[source] SummaryError),
    #[error("unique identifier")]
    UniqueIdentifier(#[source] UniqueIdentifierError),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1>
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, derive_builder::Builder)]
pub struct Event {
    dtstamp: DateTimeStamp,
    uid: UniqueIdentifier,
    dtstart: DateTimeStart,
    class: Option<Classification>,
    summary: Option<Summary>,
    dtend: Option<DateTimeEnd>,
    categories: Vec<Categories>,
}

impl From<Event> for String {
    fn from(value: Event) -> Self {
        let mut lines = vec![];
        lines.push("BEGIN:VEVENT\r\n".to_owned());
        lines.push(String::from(value.uid));
        lines.push(String::from(value.dtstamp));
        lines.push(String::from(value.dtstart));
        if let Some(dtend) = value.dtend {
            lines.push(String::from(dtend));
        }
        if let Some(summary) = value.summary {
            lines.push(String::from(summary));
        }
        if let Some(class) = value.class {
            lines.push(String::from(class));
        }
        for categories in value.categories {
            lines.push(String::from(categories));
        }
        lines.push("END:VEVENT\r\n".to_owned());
        lines.join("")
    }
}

impl TryFrom<String> for Event {
    type Error = EventError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("BEGIN:VEVENT\r\n") && value.ends_with("END:VEVENT\r\n") {
            let mut lines = value
                .trim_start_matches("BEGIN:VEVENT\r\n")
                .trim_end_matches("END:VEVENT\r\n")
                .split("\r\n")
                .collect::<Vec<&str>>();
            lines.pop();
            let mut builder = EventBuilder::default();
            for line in lines {
                if line.starts_with("UID:") {
                    let uid = UniqueIdentifier::try_from(format!("{}\r\n", line))
                        .map_err(ErrorInner::UniqueIdentifier)?;
                    builder.uid(uid);
                } else if line.starts_with("DTSTAMP:") {
                    let dtstamp = DateTimeStamp::try_from(format!("{}\r\n", line))
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
        assert_eq!(String::from(Event::try_from(s.clone())?), s);

        Ok(())
    }
}
