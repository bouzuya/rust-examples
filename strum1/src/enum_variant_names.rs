// <https://docs.rs/strum/0.24.1/strum/derive.EnumVariantNames.html>
// impl strum::VariantNames for X { ... } を追加できる。

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumVariantNames)]
enum E1 {
    A1,
    B1,
    C1,
}

#[cfg(test)]
mod tests {
    use strum::VariantNames;

    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1::VARIANTS, ["A1", "B1", "C1"]);
    }
}
