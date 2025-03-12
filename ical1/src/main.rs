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
