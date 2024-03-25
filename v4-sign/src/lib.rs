// ?=!#$&'()*+,:;@[]"
fn percent_encode(s: &str) -> String {
    let mut encoded = String::new();
    for c in s.chars() {
        match c {
            '!' => encoded.push_str("%21"),
            '"' => encoded.push_str("%22"),
            '#' => encoded.push_str("%23"),
            '$' => encoded.push_str("%24"),
            '&' => encoded.push_str("%26"),
            '\'' => encoded.push_str("%27"),
            '(' => encoded.push_str("%28"),
            ')' => encoded.push_str("%29"),
            '*' => encoded.push_str("%2A"),
            '+' => encoded.push_str("%2B"),
            ',' => encoded.push_str("%2C"),
            ':' => encoded.push_str("%3A"),
            ';' => encoded.push_str("%3B"),
            '=' => encoded.push_str("%3D"),
            '?' => encoded.push_str("%3F"),
            '@' => encoded.push_str("%40"),
            '[' => encoded.push_str("%5B"),
            ']' => encoded.push_str("%5D"),
            _ => encoded.push(c),
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode() {
        assert_eq!(
            percent_encode(r#"?=!#$&'()*+,:;@[]""#),
            "%3F%3D%21%23%24%26%27%28%29%2A%2B%2C%3A%3B%40%5B%5D%22"
        );
    }

    #[test]
    fn test_chrono() -> anyhow::Result<()> {
        let date_time =
            chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339("2020-01-02T03:04:05Z")?
                .naive_utc()
                .and_utc();
        let date = date_time.format("%Y%m%d").to_string();
        let x_goog_date = date_time.format("%Y%m%dT%H%M%SZ").to_string();
        assert_eq!(date, "20200102");
        assert_eq!(x_goog_date, "20200102T030405Z");
        Ok(())
    }
}
