use crate::i_calendar::property_parameters::{IanaParam, XParam};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OtherParam {
    IanaParam(IanaParam),
    XParam(XParam),
}

impl OtherParam {
    pub(in crate::i_calendar) fn to_escaped(&self) -> String {
        match self {
            Self::IanaParam(p) => p.to_escaped(),
            Self::XParam(p) => p.to_escaped(),
        }
    }
}

impl From<IanaParam> for OtherParam {
    fn from(p: IanaParam) -> Self {
        Self::IanaParam(p)
    }
}

impl From<XParam> for OtherParam {
    fn from(value: XParam) -> Self {
        Self::XParam(value)
    }
}
