mod i_calendar;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut cal = ical::generator::IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("prodid1")
        .build();

    let event = ical::generator::IcalEventBuilder::tzid("Asia/Tokyo")
        .uid("c2b4ad47-168c-4a3b-81e7-e06bf5fd3091")
        .changed("20250309T210000")
        .start("20250309T213000")
        .end("20250309T220000")
        .set(ical::property::Property {
            name: String::from("SUMMARY"),
            value: Some("Summary1".into()),
            params: None,
        })
        .build();
    cal.events.push(event);

    assert_eq!(
        ical::generator::Emitter::generate(&cal),
        r#"BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:prodid1
BEGIN:VEVENT
UID:c2b4ad47-168c-4a3b-81e7-e06bf5fd3091
DTSTAMP;TZID=Asia/Tokyo:20250309T210000
DTSTART;TZID=Asia/Tokyo:20250309T213000
DTEND;TZID=Asia/Tokyo:20250309T220000
SUMMARY:Summary1
END:VEVENT
END:VCALENDAR
"#
        .replace("\n", "\r\n")
    );

    Ok(())
}

fn fold<I>(mut iter: I) -> String
where
    I: Iterator<Item = char>,
{
    let mut lines = String::new();
    let mut line_len = 0;
    while let Some(c) = iter.next() {
        if c == '\r' {
            match iter.next() {
                Some('\n') => {
                    lines.push('\r');
                    lines.push('\n');
                    line_len = 0;
                }
                _ => {
                    // CR without LF
                    unreachable!();
                }
            }
        } else {
            if line_len + c.len_utf8() > 75 {
                lines.push('\r');
                lines.push('\n');
                lines.push(' ');
                line_len = 1;
            }
            lines.push(c);
            line_len += c.len_utf8();
        }
    }
    assert_eq!(line_len, 0);
    lines
}

fn unfold<I>(mut iter: I) -> String
where
    I: Iterator<Item = char>,
{
    let mut lines = String::new();
    let mut crlf = false;
    while let Some(c) = iter.next() {
        if crlf {
            crlf = false;
            if c == ' ' || c == '\t' {
                // Unfolding
            } else {
                lines.push('\r');
                lines.push('\n');
                lines.push(c);
            }
        } else {
            if c == '\r' {
                match iter.next() {
                    Some('\n') => {
                        // CRLF
                        crlf = true;
                    }
                    _ => {
                        // CR without LF
                        unreachable!();
                    }
                }
            } else {
                lines.push(c);
            }
        }
    }
    assert!(crlf);
    lines.push('\r');
    lines.push('\n');
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold() {
        assert_eq!(
            fold("DESCRIPTION:This is a long description that exists on a long line.\r\n".chars()),
            "DESCRIPTION:This is a long description that exists on a long line.\r\n"
        );

        assert_eq!(
            fold(
                "1234567890:234567890123456789012345678901234567890123456789012345678901234567890\r\n"
                    .chars()
            ),
            [
                "1234567890:2345678901234567890123456789012345678901234567890123456789012345\r\n",
                " 67890\r\n"
            ]
            .join("")
        );
    }

    #[test]
    fn test_unfold() {
        assert_eq!(
            unfold(
                "DESCRIPTION:This is a long description that exists on a long line.\r\n".chars()
            ),
            "DESCRIPTION:This is a long description that exists on a long line.\r\n"
        );
        assert_eq!(
            unfold(
                "DESCRIPTION:This is a lo\r\n ng description\r\n  that exists on a long line.\r\n"
                    .chars()
            ),
            "DESCRIPTION:This is a long description that exists on a long line.\r\n"
        );

        assert_eq!(
            unfold(
                [
                    "1234567890:2345678901234567890123456789012345678901234567890123456789012345\r\n",
                    " 67890\r\n"
                ].join("").chars()
            ),
            "1234567890:234567890123456789012345678901234567890123456789012345678901234567890\r\n"
        );
    }

    #[test]
    fn test_i_calendar_stream() -> anyhow::Result<()> {
        use i_calendar::{
            CalendarScale, Event, ICalendarObject, ICalendarStream, Method, ProductIdentifier,
            UniqueIdentifier, Version,
        };
        let i_calendar_stream = ICalendarStream::builder()
            .add_object(
                ICalendarObject::builder()
                    .prodid(ProductIdentifier::from_value(
                        "-//ABC Corporation//NONSGML My Product//EN",
                    )?)
                    .version(Version::from_value("2.0")?)
                    .calscale(CalendarScale::from_value("GREGORIAN")?)
                    .method(Method::from_value("PUBLISH")?)
                    .add_component(
                        Event::builder()
                            .uid(UniqueIdentifier::from_value(
                                "19970901T130000Z-123401@example.com",
                            )?)
                            .build()?,
                        // Event::try_from(
                        //     [
                        //         "BEGIN:VEVENT\r\n",
                        //         "UID:19970901T130000Z-123401@example.com\r\n",
                        //         "DTSTAMP:19970901T130000Z\r\n",
                        //         "DTSTART:19970903T163000Z\r\n",
                        //         "DTEND:19970903T190000Z\r\n",
                        //         "SUMMARY:Annual Employee Review\r\n",
                        //         "CLASS:PRIVATE\r\n",
                        //         "CATEGORIES:BUSINESS,HUMAN RESOURCES\r\n",
                        //         "END:VEVENT\r\n",
                        //     ]
                        //     .join(""),
                        // )?,
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
}
