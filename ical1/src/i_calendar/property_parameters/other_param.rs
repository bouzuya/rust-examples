use crate::i_calendar::property_parameters::{IanaParam, XParam};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::i_calendar) enum OtherParam {
    IanaParam(IanaParam),
    XParam(XParam),
}
