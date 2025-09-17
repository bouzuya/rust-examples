use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, parse_display::Display, parse_display::FromStr)]
#[display(style = "snake_case")]
enum MyEnum {
    One,
    #[display("2")]
    Two,
    Three,
    FourTeen,
}

// エラー型が parse_display::ParseError
fn main() -> Result<(), parse_display::ParseError> {
    for (s, o) in [
        ("one", MyEnum::One),
        ("2", MyEnum::Two),
        ("three", MyEnum::Three),
        ("four_teen", MyEnum::FourTeen),
    ] {
        assert_eq!(MyEnum::from_str(s)?, o);
        assert_eq!(o.to_string(), s);
    }
    Ok(())
}
