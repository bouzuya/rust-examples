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
}
