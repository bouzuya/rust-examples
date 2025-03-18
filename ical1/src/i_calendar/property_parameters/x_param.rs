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

    pub(in crate::i_calendar) fn to_escaped(&self) -> String {
        let mut s = String::new();
        s.push_str(self.name.to_escaped().as_str());
        s.push('=');
        let mut iter = self.value.iter();
        if let Some(v) = iter.next() {
            s.push_str(v.to_escaped().as_str());
        }
        for v in iter {
            s.push(',');
            s.push_str(v.to_escaped().as_str());
        }
        s
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
        assert_eq!(x_param.to_escaped(), "X-NAME=param-value");

        let name = XName::from_unescaped("X-NAME")?;
        let value = vec![
            ParamValue::from_unescaped("param-value1")?,
            ParamValue::from_unescaped("param-value2")?,
        ];
        let x_param = XParam::new(name.clone(), value.clone())?;
        assert_eq!(x_param.name(), &name);
        assert_eq!(x_param.value(), &value);
        assert_eq!(x_param.to_escaped(), "X-NAME=param-value1,param-value2");

        assert!(XParam::new(name.clone(), vec![]).is_err());

        Ok(())
    }
}
