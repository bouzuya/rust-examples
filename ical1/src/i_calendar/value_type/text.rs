#[derive(Debug, thiserror::Error)]
#[error("text")]
pub struct TextError {
    // TODO: improve error message
    _private: (),
}

/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>
///
/// text       = *(TSAFE-CHAR / ":" / DQUOTE / ESCAPED-CHAR)
///    ; Folded according to description above
/// ESCAPED-CHAR = ("\\" / "\;" / "\," / "\N" / "\n")
///    ; \\ encodes \, \N or \n encodes newline
///    ; \; encodes ;, \, encodes ,
/// TSAFE-CHAR = WSP / %x21 / %x23-2B / %x2D-39 / %x3C-5B /
///              %x5D-7E / NON-US-ASCII
///    ; Any character except CONTROLs not needed by the current
///    ; character set, DQUOTE, ";", ":", "\", ","
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Text(String);

impl Text {
    pub(in crate::i_calendar) fn from_string(s: String) -> Result<Self, TextError> {
        Ok(Self(s))
    }

    pub fn from_unescaped(s: &str) -> Result<Self, TextError> {
        let mut t = String::new();
        for c in s.chars() {
            if c.is_ascii_control() && !(c == '\n' || c == '\t') {
                return Err(TextError { _private: () });
            }
            match c {
                '\\' | ';' | ',' => {
                    t.push('\\');
                    t.push(c);
                }
                '\n' => {
                    t.push_str("\\n");
                }
                c => {
                    t.push(c);
                }
            }
        }
        Ok(Self(t))
    }

    pub(in crate::i_calendar) fn into_string(self) -> String {
        self.0
    }

    pub fn to_unescaped(&self) -> String {
        let mut s = String::new();
        let mut iter = self.0.chars();
        while let Some(c) = iter.next() {
            match c {
                '\\' => match iter.next() {
                    None => unreachable!(),
                    Some(c) => match c {
                        '\\' => {
                            s.push('\\');
                        }
                        ';' => {
                            s.push(';');
                        }
                        ',' => {
                            s.push(',');
                        }
                        'n' | 'N' => {
                            s.push('\n');
                        }
                        _ => unreachable!(),
                    },
                },
                c => {
                    s.push(c);
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        fn assert_fn<T: Clone + Eq + Ord + PartialEq + PartialOrd>() {}
        assert_fn::<Text>();

        let s = "Project XYZ Final Review\\nConference Room - 3B\\nCome Prepared.".to_owned();
        assert_eq!(Text::from_string(s.clone())?.into_string(), s);

        let s = ":\"\\;,\n \t";
        assert_eq!(
            Text::from_unescaped(s)?.into_string(),
            ":\"\\\\\\;\\,\\n \t"
        );
        assert_eq!(Text::from_unescaped(s)?.to_unescaped(), s);
        Ok(())
    }
}
