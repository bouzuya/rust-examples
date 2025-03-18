use crate::i_calendar::property_parameters::{ParamValue, XName};

#[derive(Debug, thiserror::Error)]
#[error("x-param")]
pub struct XParamError {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XParam {
    name: XName,
    // 1*param-value
    value: Vec<ParamValue>,
}

impl XParam {
    pub fn new(name: XName, value: Vec<ParamValue>) -> Result<Self, XParamError> {
        if value.is_empty() {
            return Err(XParamError { _private: () });
        }

        Ok(Self { name, value })
    }

    pub fn name(&self) -> &XName {
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
        assert_fn::<XParam>();

        let name = XName::from_unescaped("X-NAME")?;
        let value = vec![ParamValue::from_unescaped("param-value")?];
        let x_param = XParam::new(name.clone(), value.clone())?;
        assert_eq!(x_param.name(), &name);
        assert_eq!(x_param.value(), &value);

        assert!(XParam::new(name.clone(), vec![]).is_err());

        Ok(())
    }
}
