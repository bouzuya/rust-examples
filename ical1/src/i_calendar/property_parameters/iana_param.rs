use crate::i_calendar::property_parameters::{IanaToken, ParamValue};

#[derive(Debug, thiserror::Error)]
#[error("iana-param")]
pub struct IanaParamError {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IanaParam {
    name: IanaToken,
    // 1*param-value
    value: Vec<ParamValue>,
}

impl IanaParam {
    pub fn new(name: IanaToken, value: Vec<ParamValue>) -> Result<Self, IanaParamError> {
        if value.is_empty() {
            return Err(IanaParamError { _private: () });
        }

        Ok(Self { name, value })
    }

    pub fn name(&self) -> &IanaToken {
        &self.name
    }

    pub fn value(&self) -> &[ParamValue] {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<IanaParam>();

        let name = IanaToken::from_unescaped("IANA-TOKEN")?;
        let value = vec![ParamValue::from_unescaped("param-value")?];
        let iana_param = IanaParam::new(name.clone(), value.clone())?;
        assert_eq!(iana_param.name(), &name);
        assert_eq!(iana_param.value(), &value);

        assert!(IanaParam::new(name.clone(), vec![]).is_err());

        Ok(())
    }
}
