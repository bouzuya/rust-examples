#[test]
fn test_i_calendar_stream() -> Result<(), ical1::Error> {
    use ical1::{
        CalendarScale, Categories, Classification, DateTimeEnd, DateTimeStamp, DateTimeStart,
        Event, ICalendarObject, ICalendarStream, Method, ProductIdentifier, Summary, Text,
        UniqueIdentifier, Version,
    };
    use std::str::FromStr as _;

    let i_calendar_stream = ICalendarStream::builder()
        .add_object(
            ICalendarObject::builder()
                .prodid(ProductIdentifier::from_value(
                    "-//ABC Corporation//NONSGML My Product//EN",
                )?)
                .version(Version::new(Text::from_str("2.0")?)?)
                .calscale(CalendarScale::new(Text::from_str("GREGORIAN")?)?)
                .method(Method::new(Text::from_str("PUBLISH")?)?)
                .add_component(
                    Event::builder()
                        .uid(UniqueIdentifier::from_value(
                            "19970901T130000Z-123401@example.com",
                        )?)
                        .dtstamp(DateTimeStamp::from_value("19970901T130000Z")?)
                        .dtstart(DateTimeStart::from_value("19970903T163000Z")?)
                        .dtend(DateTimeEnd::from_value("19970903T190000Z")?)
                        .summary(Summary::from_value("Annual Employee Review")?)
                        .class(Classification::from_value("PRIVATE")?)
                        .add_categories(Categories::from_value("BUSINESS,HUMAN RESOURCES")?)
                        .build()?,
                )
                .build()?,
        )
        .build()?;
    assert_eq!(
        i_calendar_stream.to_string(),
        [
            "BEGIN:VCALENDAR\r\n",
            "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n",
            "VERSION:2.0\r\n",
            "CALSCALE:GREGORIAN\r\n",
            "METHOD:PUBLISH\r\n",
            "BEGIN:VEVENT\r\n",
            "UID:19970901T130000Z-123401@example.com\r\n",
            "DTSTAMP:19970901T130000Z\r\n",
            "DTSTART:19970903T163000Z\r\n",
            "DTEND:19970903T190000Z\r\n",
            "SUMMARY:Annual Employee Review\r\n",
            "CLASS:PRIVATE\r\n",
            "CATEGORIES:BUSINESS,HUMAN RESOURCES\r\n",
            "END:VEVENT\r\n",
            "END:VCALENDAR\r\n"
        ]
        .join("")
    );
    Ok(())
}

#[test]
fn test_i_calendar_stream_minimum() -> Result<(), ical1::Error> {
    use ical1::{
        DateTimeStamp, DateTimeStart, Event, ICalendarObject, ICalendarStream, ProductIdentifier,
        Text, UniqueIdentifier, Version,
    };

    let i_calendar_stream = ICalendarStream::builder()
        .add_object(
            ICalendarObject::builder()
                .prodid(ProductIdentifier::from_value(
                    "-//ABC Corporation//NONSGML My Product//EN",
                )?)
                .version(Version::new(Text::from_unescaped("2.0")?)?)
                .add_component(
                    Event::builder()
                        .uid(UniqueIdentifier::from_value(
                            "19970901T130000Z-123401@example.com",
                        )?)
                        .dtstamp(DateTimeStamp::from_value("19970901T130000Z")?)
                        .dtstart(DateTimeStart::from_value("19970903T163000Z")?)
                        .build()?,
                )
                .build()?,
        )
        .build()?;
    assert_eq!(
        i_calendar_stream.to_string(),
        [
            "BEGIN:VCALENDAR\r\n",
            "PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n",
            "VERSION:2.0\r\n",
            "BEGIN:VEVENT\r\n",
            "UID:19970901T130000Z-123401@example.com\r\n",
            "DTSTAMP:19970901T130000Z\r\n",
            "DTSTART:19970903T163000Z\r\n",
            "END:VEVENT\r\n",
            "END:VCALENDAR\r\n"
        ]
        .join("")
    );
    Ok(())
}
