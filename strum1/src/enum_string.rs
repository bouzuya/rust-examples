// <https://docs.rs/strum_macros/0.24.1/strum_macros/derive.EnumString.html>
/// impl std::str::FromStr for X と
/// impl std::convert::TryFrom<&str> for X を追加できる

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumString)]
enum E1 {
    A1,
    #[strum(serialize = "b")]
    B1,
    C1(usize),
    D1 {
        // TODO: Default が取れない場合
        s: String,
    },
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn e1_test() -> anyhow::Result<()> {
        assert_eq!(E1::A1, E1::from_str("A1")?);
        assert_eq!(E1::A1, E1::try_from("A1")?);
        assert!(E1::from_str("B1").is_err());
        assert_eq!(E1::B1, E1::try_from("b")?);
        assert_eq!(E1::C1(0), E1::try_from("C1")?);
        assert_eq!(
            E1::D1 {
                s: String::default()
            },
            E1::try_from("D1")?
        );
        Ok(())
    }
}
