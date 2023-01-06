// <https://docs.rs/strum_macros/0.24.1/strum_macros/derive.EnumCount.html>
// impl strum::EnumCount for X を追加できる
// EnumCount は COUNT という定数を持っている

#[allow(dead_code)]
#[derive(strum::EnumCount)]
enum E1 {
    A1,
    B1,
    C1,
}

#[cfg(test)]
mod tests {
    // これがないと E1::COUNT とできない
    use strum::EnumCount;

    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1::COUNT, 3);
    }
}
