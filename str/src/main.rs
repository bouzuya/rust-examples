fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_split() {
        let mut split: std::str::Split<char> = "foo\nbar".split('\n');
        assert_eq!(split.nth(0), Some("foo"));

        let split = "1 2 3".split(' ').collect::<Vec<&str>>();
        assert_eq!(split, ["1", "2", "3"]);

        let split = "000".split('0').collect::<Vec<&str>>();
        assert_eq!(split, ["", "", "", ""]);

        let split: std::str::Split<&str> = "abc123def123ghi".split("123");
        let split = split.collect::<Vec<&str>>();
        assert_eq!(split, ["abc", "def", "ghi"]);
    }
}
