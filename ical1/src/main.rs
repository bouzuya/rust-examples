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

fn fold<I>(iter: I) -> Option<Vec<String>>
where
    I: Iterator<Item = char>,
{
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut cr = false;
    for c in iter {
        if cr {
            cr = false;
            if c == '\n' {
                lines.push(line);
                line = String::new();
                continue;
            }
            // CR without LF
            return None;
        } else {
            if c == '\r' {
                cr = true;
                continue;
            }
            if line.len() + c.len_utf8() > 75 {
                lines.push(line);
                line = String::new();
                line.push(' ');
            }
            line.push(c);
        }
    }
    if cr {
        // CR at the end
        return None;
    }
    lines.push(line);
    Some(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let chars = "DESCRIPTION:This is a long description that exists on a long line.";
        assert_eq!(
            fold(chars.chars()),
            Some(vec![
                "DESCRIPTION:This is a long description that exists on a long line.".to_owned()
            ])
        );

        let chars =
            "1234567890:234567890123456789012345678901234567890123456789012345678901234567890";
        assert_eq!(
            fold(chars.chars()),
            Some(vec![
                "1234567890:2345678901234567890123456789012345678901234567890123456789012345"
                    .to_owned(),
                " 67890".to_owned(),
            ])
        );
    }
}
