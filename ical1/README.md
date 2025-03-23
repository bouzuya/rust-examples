# ical1

[crates:ical] の例と、 Internet Calendaring and Scheduling Core Object Specification (iCalendar) (RFC5545) の理解のためのコードを記載している。

lib crate は iCalendar の builder を提供し、 ics を文字列として出力できる。

[crates:ical]: https://crates.io/crates/ical

## Usage

```rust
fn main() -> Result<(), ical1::Error> {
    use ical1::{
        CalendarScale, Categories, Classification, DateTimeEnd, DateTimeStamp, DateTimeStart,
        Event, ICalendarObject, ICalendarStream, Method, ProductIdentifier, Summary, Text,
        UniqueIdentifier, Version,
    };
    use std::str::FromStr as _;

    let i_calendar_stream = ICalendarStream::builder()
        .add_object(
            ICalendarObject::builder()
                .prodid(ProductIdentifier::new(Text::from_unescaped(
                    "-//ABC Corporation//NONSGML My Product//EN",
                )?)?)
                .version(Version::new(Text::from_str("2.0")?)?)
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
```
