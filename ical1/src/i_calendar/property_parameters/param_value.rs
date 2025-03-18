#[derive(Debug, thiserror::Error)]
#[error("param-value")]
pub struct ParamValueError {
    _private: (),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.1>
/// param-value   = paramtext / quoted-string
/// paramtext     = *SAFE-CHAR
/// quoted-string = DQUOTE *QSAFE-CHAR DQUOTE
/// QSAFE-CHAR    = WSP / %x21 / %x23-7E / NON-US-ASCII
/// ; Any character except CONTROL and DQUOTE
/// SAFE-CHAR     = WSP / %x21 / %x23-2B / %x2D-39 / %x3C-7E
///               / NON-US-ASCII
/// ; Any character except CONTROL, DQUOTE, ";", ":", ","
/// NON-US-ASCII  = UTF8-2 / UTF8-3 / UTF8-4
/// ; UTF8-2, UTF8-3, and UTF8-4 are defined in [RFC3629]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParamValue {
    s: String,
    safe_char: bool,
}

impl ParamValue {
    fn from_unescaped(s: &str) -> Result<Self, ParamValueError> {
        let mut safe_char = true;
        for c in s.chars() {
            if (c.is_ascii_control() && c != '\t') || c == '"' {
                return Err(ParamValueError { _private: () });
            }
            if c == ';' || c == ':' || c == ',' {
                safe_char = false;
            }
        }
        Ok(Self {
            s: s.to_owned(),
            safe_char,
        })
    }

    fn to_escaped(&self) -> String {
        if self.safe_char {
            self.s.clone()
        } else {
            format!("\"{}\"", self.s)
        }
    }

    fn to_unescaped(&self) -> String {
        self.s.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + PartialEq>() {}
        assert_fn::<ParamValue>();

        let s = "text";
        assert_eq!(ParamValue::from_unescaped(s)?.to_unescaped(), s);
        assert_eq!(ParamValue::from_unescaped(s)?.to_escaped(), s);

        let s = ";:,";
        assert_eq!(ParamValue::from_unescaped(s)?.to_unescaped(), ";:,");
        assert_eq!(ParamValue::from_unescaped(s)?.to_escaped(), "\";:,\"");

        let s = "text\"";
        assert!(ParamValue::from_unescaped(s).is_err());

        let s = "text\n";
        assert!(ParamValue::from_unescaped(s).is_err());

        Ok(())
    }
}
