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

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.4>
/// icalstream = 1*icalobject
/// iCalendar stream
struct ICalendarStream(Vec<ICalendarObject>);

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.4>
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6>
/// icalobject = "BEGIN" ":" "VCALENDAR" CRLF
///              icalbody
///              "END" ":" "VCALENDAR" CRLF
/// icalbody   = calprops component
/// calprops   = *(
///            ;
///            ; The following are REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            prodid / version /
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            calscale / method /
///            ;
///            ; The following are OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            x-prop / iana-prop
///            ;
///            )
/// component  = 1*(eventc / todoc / journalc / freebusyc /
///              timezonec / iana-comp / x-comp)
/// iana-comp  = "BEGIN" ":" iana-token CRLF
///              1*contentline
///              "END" ":" iana-token CRLF
/// x-comp     = "BEGIN" ":" x-name CRLF
///              1*contentline
///              "END" ":" x-name CRLF
/// iCalendar Object
struct ICalendarObject {
    calprops: Vec<CalendarProperty>,
    component: Vec<CalendarComponent>,
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6>
/// calprops
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7>
enum CalendarProperty {
    /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.1>
    /// calscale
    /// Calendar Scale
    CalendarScale(
        // TODO
    ),
    /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.2>
    /// method
    /// Method
    Method(
        // TODO
    ),
    /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.3>
    /// prodid
    /// Product Identifier
    ProductIdentifier(
        // TODO
    ),
    /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.4>
    /// version
    /// Version
    Version(
        // TODO
    ),

    /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.1>
    /// iana-prop
    /// 3.8. Component Properties
    /// 3.8.8. Miscellaneous Component Properties
    /// 3.8.8.1. IANA Properties
    IanaProperties(
        // TODO
    ),
    /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.2>
    /// x-prop
    /// 3.8. Component Properties
    /// 3.8.8. Miscellaneous Component Properties
    /// 3.8.8.2. Non-Standard Properties
    NonStandardProperties(
        // TODO
    ),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6>
/// component  = 1*(eventc / todoc / journalc / freebusyc /
///              timezonec / iana-comp / x-comp)
/// iana-comp  = "BEGIN" ":" iana-token CRLF
///              1*contentline
///              "END" ":" iana-token CRLF
/// x-comp     = "BEGIN" ":" x-name CRLF
///              1*contentline
///              "END" ":" x-name CRLF
enum CalendarComponent {
    /// Event Component
    /// eventc
    Event(
        // TODO
    ),
    /// To-Do Component
    /// todoc
    Todo(
        // TODO
    ),
    /// Journal Component
    /// journalc
    Journal(
        // TODO
    ),
    /// Free/Busy Component
    /// freebusyc
    Freebusy(
        // TODO
    ),
    /// Time Zone Component
    /// timezonec
    Timezone(
        // TODO
    ),
    /// iana-comp  = "BEGIN" ":" iana-token CRLF
    ///              1*contentline
    ///              "END" ":" iana-token CRLF
    IanaComp(
        // TODO
    ),
    /// x-comp     = "BEGIN" ":" x-name CRLF
    ///              1*contentline
    ///              "END" ":" x-name CRLF
    XComp(
        // TODO
    ),
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
}
