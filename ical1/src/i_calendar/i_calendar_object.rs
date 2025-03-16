use crate::i_calendar::{calendar_components, calendar_properties};

use super::utils::fold;

mod private {
    use super::CalendarComponent;

    pub trait IntoCalendarComponentSealed {
        fn into_calendar_component(self) -> CalendarComponent;
    }
}

pub trait IntoCalendarComponent: private::IntoCalendarComponentSealed {}

trait WriteTo {
    fn write_to<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result;
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.4>
/// icalstream = 1*icalobject
/// iCalendar stream
pub struct ICalendarStream(Vec<ICalendarObject>);

impl std::fmt::Display for ICalendarStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.write_to(&mut s)?;
        fold(s.chars()).fmt(f)
    }
}

impl ICalendarStream {
    pub fn builder() -> ICalendarStreamBuilder {
        ICalendarStreamBuilder::new()
    }

    fn new(i_calendar_objects: Vec<ICalendarObject>) -> Self {
        assert!(!i_calendar_objects.is_empty());
        Self(i_calendar_objects)
    }
}

impl WriteTo for ICalendarStream {
    fn write_to<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        for i_calendar_object in &self.0 {
            i_calendar_object.write_to(w)?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("iCalendar stream builder")]
pub struct ICalendarStreamBuilderError;

pub struct ICalendarStreamBuilder(Vec<ICalendarObject>);

impl ICalendarStreamBuilder {
    fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_object(mut self, object: ICalendarObject) -> Self {
        self.0.push(object);
        self
    }

    pub fn build(self) -> Result<ICalendarStream, ICalendarStreamBuilderError> {
        if self.0.is_empty() {
            return Err(ICalendarStreamBuilderError);
        }

        Ok(ICalendarStream::new(self.0))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("iCalendar object")]
pub struct ICalendarObjectError;

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.4>
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6>
/// icalobject = "BEGIN" ":" "VCALENDAR" CRLF
///              icalbody
///              "END" ":" "VCALENDAR" CRLF
/// icalbody   = calprops component
/// component  = 1*(eventc / todoc / journalc / freebusyc /
///              timezonec / iana-comp / x-comp)
/// iana-comp  = "BEGIN" ":" iana-token CRLF
///              1*contentline
///              "END" ":" iana-token CRLF
/// x-comp     = "BEGIN" ":" x-name CRLF
///              1*contentline
///              "END" ":" x-name CRLF
/// iCalendar Object
pub struct ICalendarObject {
    calprops: CalendarProperties,
    component: Vec<CalendarComponent>,
}

impl ICalendarObject {
    pub fn builder() -> ICalendarObjectBuilder {
        ICalendarObjectBuilder::new()
    }

    fn from_builder(builder: ICalendarObjectBuilder) -> Result<Self, ICalendarObjectError> {
        if builder.component.is_empty() {
            return Err(ICalendarObjectError);
        }
        match (builder.prodid, builder.version) {
            (Some(prodid), Some(version)) => Ok(Self {
                calprops: CalendarProperties {
                    prodid,
                    version,
                    calscale: builder.calscale,
                    method: builder.method,
                },
                component: builder.component,
            }),
            _ => Err(ICalendarObjectError),
        }
    }
}

impl WriteTo for ICalendarObject {
    fn write_to<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        w.write_str("BEGIN:VCALENDAR\r\n")?;
        self.calprops.write_to(w)?;
        for component in &self.component {
            component.write_to(w)?;
        }
        w.write_str("END:VCALENDAR\r\n")?;
        Ok(())
    }
}

pub struct ICalendarObjectBuilder {
    // calprops
    prodid: Option<calendar_properties::ProductIdentifier>,
    version: Option<calendar_properties::Version>,
    calscale: Option<calendar_properties::CalendarScale>,
    method: Option<calendar_properties::Method>,
    // x_prop: Vec<NonStandardProperty>,
    // iana_prop: Vec<IanaProperty>,

    // component
    component: Vec<CalendarComponent>,
}

impl ICalendarObjectBuilder {
    fn new() -> Self {
        Self {
            prodid: None,
            version: None,
            calscale: None,
            method: None,
            component: Vec::new(),
        }
    }

    pub fn build(self) -> Result<ICalendarObject, ICalendarObjectError> {
        ICalendarObject::from_builder(self)
    }

    pub fn add_component<C: IntoCalendarComponent>(mut self, component: C) -> Self {
        self.component.push(component.into_calendar_component());
        self
    }

    pub fn calscale(mut self, calscale: calendar_properties::CalendarScale) -> Self {
        self.calscale = Some(calscale);
        self
    }

    pub fn method(mut self, method: calendar_properties::Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn prodid(mut self, prodid: calendar_properties::ProductIdentifier) -> Self {
        self.prodid = Some(prodid);
        self
    }

    pub fn version(mut self, version: calendar_properties::Version) -> Self {
        self.version = Some(version);
        self
    }
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6>
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
struct CalendarProperties {
    prodid: calendar_properties::ProductIdentifier,
    version: calendar_properties::Version,
    calscale: Option<calendar_properties::CalendarScale>,
    method: Option<calendar_properties::Method>,
    // x_prop: Vec<NonStandardProperty>,
    // iana_prop: Vec<IanaProperty>,
}

impl WriteTo for CalendarProperties {
    fn write_to<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        w.write_str(self.prodid.clone().into_string().as_str())?;
        w.write_str(self.version.clone().into_string().as_str())?;
        if let Some(calscale) = &self.calscale {
            w.write_str(calscale.clone().into_string().as_str())?;
        }
        if let Some(method) = &self.method {
            w.write_str(method.clone().into_string().as_str())?;
        }
        Ok(())
    }
}

// enum CalendarProperty {
//     /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.1>
//     /// iana-prop
//     /// 3.8. Component Properties
//     /// 3.8.8. Miscellaneous Component Properties
//     /// 3.8.8.1. IANA Properties
//     IanaProperties(
//         // TODO
//     ),
//     /// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.2>
//     /// x-prop
//     /// 3.8. Component Properties
//     /// 3.8.8. Miscellaneous Component Properties
//     /// 3.8.8.2. Non-Standard Properties
//     NonStandardProperties(
//         // TODO
//     ),
// }

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6>
/// component  = 1*(eventc / todoc / journalc / freebusyc /
///              timezonec / iana-comp / x-comp)
/// iana-comp  = "BEGIN" ":" iana-token CRLF
///              1*contentline
///              "END" ":" iana-token CRLF
/// x-comp     = "BEGIN" ":" x-name CRLF
///              1*contentline
///              "END" ":" x-name CRLF
pub struct CalendarComponent(CalendarComponentInner);

enum CalendarComponentInner {
    /// Event Component
    /// eventc
    Event(calendar_components::Event),
    // /// To-Do Component
    // /// todoc
    // Todo(
    //     // TODO
    // ),
    // /// Journal Component
    // /// journalc
    // Journal(
    //     // TODO
    // ),
    // /// Free/Busy Component
    // /// freebusyc
    // Freebusy(
    //     // TODO
    // ),
    // /// Time Zone Component
    // /// timezonec
    // Timezone(
    //     // TODO
    // ),
    // /// iana-comp  = "BEGIN" ":" iana-token CRLF
    // ///              1*contentline
    // ///              "END" ":" iana-token CRLF
    // IanaComp(
    //     // TODO
    // ),
    // /// x-comp     = "BEGIN" ":" x-name CRLF
    // ///              1*contentline
    // ///              "END" ":" x-name CRLF
    // XComp(
    //     // TODO
    // ),
}

impl private::IntoCalendarComponentSealed for CalendarComponent {
    fn into_calendar_component(self) -> CalendarComponent {
        self
    }
}

impl IntoCalendarComponent for CalendarComponent {}

impl private::IntoCalendarComponentSealed for calendar_components::Event {
    fn into_calendar_component(self) -> CalendarComponent {
        CalendarComponent(CalendarComponentInner::Event(self))
    }
}

impl IntoCalendarComponent for calendar_components::Event {}

impl WriteTo for CalendarComponent {
    fn write_to<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        match &self.0 {
            CalendarComponentInner::Event(event) => {
                w.write_str(event.clone().into_string().as_str())?
            }
        }
        Ok(())
    }
}
