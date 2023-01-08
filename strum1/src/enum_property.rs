// <https://docs.rs/strum/0.24.1/strum/derive.EnumProperty.html>
// impl strum::EnumProperty for X { ... } を追加できる。
// EnumMessage のキーを任意の文字列にしたイメージ。

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumProperty)]
enum E1 {
    A1,
    #[strum(props(key1 = "B1 str1"))]
    B1,
    #[strum(props(key2 = "C1 str2"))]
    C1,
}

#[cfg(test)]
mod tests {
    use strum::EnumProperty;

    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1::A1.get_bool("key1"), None);
        assert_eq!(E1::A1.get_int("key1"), None);
        assert_eq!(E1::A1.get_str("key1"), None);
        // get_bool は提供されているが動作しない
        assert_eq!(E1::B1.get_bool("key1"), None);
        // get_int は提供されているが動作しない
        assert_eq!(E1::B1.get_int("key1"), None);
        assert_eq!(E1::B1.get_str("key1"), Some("B1 str1"));
        assert_eq!(E1::C1.get_str("key2"), Some("C1 str2"));
    }
}
