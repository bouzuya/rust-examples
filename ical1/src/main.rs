mod component;
mod property;
mod value_type;

fn main() {
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
"#.replace("\n", "\r\n")
    );
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
}
