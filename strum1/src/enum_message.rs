// <https://docs.rs/strum/0.24.1/strum/derive.EnumMessage.html>
// impl strum::EnumMessage for X { ... } を追加できる。
// variant にメッセージ (message / detailed_message / documentation) を関連づけられる。

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumMessage)]
enum E1 {
    A1,
    #[strum(message = "B1 message")]
    B1,
    #[strum(detailed_message = "C1 detailed message")]
    C1,
    /// D1 documentation
    D1,
    /// E1 documentation
    #[strum(message = "E1 message", detailed_message = "E1 detailed message")]
    E1,
}

#[cfg(test)]
mod tests {
    use strum::EnumMessage;

    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1::A1.get_message(), None);
        assert_eq!(E1::A1.get_detailed_message(), None);
        assert_eq!(E1::A1.get_documentation(), None);
        assert_eq!(E1::B1.get_message(), Some("B1 message"));
        assert_eq!(E1::C1.get_detailed_message(), Some("C1 detailed message"));
        assert_eq!(E1::D1.get_documentation(), Some("D1 documentation"));
        assert_eq!(E1::E1.get_message(), Some("E1 message"));
        assert_eq!(E1::E1.get_detailed_message(), Some("E1 detailed message"));
        assert_eq!(E1::E1.get_documentation(), Some("E1 documentation"));
    }
}
