// <https://docs.rs/strum_macros/0.24.1/strum_macros/derive.AsRefStr.html>
// impl std::convert::AsRef<str> for X を追加できる

#[allow(dead_code)]
#[derive(strum::AsRefStr)]
enum E1 {
    A1,
    #[strum(serialize = "b")]
    B1,
    C1(usize),
    D1 {
        s: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1::A1.as_ref(), "A1");
        assert_eq!(E1::B1.as_ref(), "b");
        assert_eq!(E1::C1(10).as_ref(), "C1");
        assert_eq!(E1::D1 { s: "s1".to_owned() }.as_ref(), "D1");
    }
}
