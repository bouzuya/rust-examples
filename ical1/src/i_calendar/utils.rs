pub(in crate::i_calendar) fn fold<I>(mut iter: I) -> String
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

pub(in crate::i_calendar) fn unfold<I>(mut iter: I) -> String
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
