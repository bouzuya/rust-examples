fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    fn anychar_test_f(s: &str) -> IResult<&str, char> {
        nom::character::complete::anychar(s)
    }

    #[test]
    fn anychar_test() {
        assert_eq!(anychar_test_f("abc"), Ok(("bc", 'a')));
    }

    fn char_test_f(s: &str) -> IResult<&str, char> {
        nom::character::complete::char('"')(s)
    }

    #[test]
    fn char_test() {
        assert_eq!(char_test_f("\""), Ok(("", '"')));
        assert_eq!(char_test_f("\"bc"), Ok(("bc", '"')));
    }

    fn fold_many0_test_f(s: &str) -> IResult<&str, String> {
        nom::multi::fold_many0(
            nom::character::complete::anychar,
            String::new(),
            |mut s, c| {
                s.push(c);
                s
            },
        )(s)
    }

    #[test]
    fn fold_many0_test() {
        assert_eq!(fold_many0_test_f("abc"), Ok(("", "abc".to_string())));
        assert_eq!(fold_many0_test_f(""), Ok(("", "".to_string())));
    }
}
