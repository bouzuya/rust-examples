// <https://docs.rs/strum/0.24.1/strum/derive.EnumIter.html>
// impl strum::IntoEnumIterator for X { ... } を追加できる。
// variant を iter できる

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumIter)]
enum E1 {
    A1,
    B1,
    C1(usize),
    D1 { s: String },
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn e1_test() {
        let mut iter = E1::iter();
        assert_eq!(iter.next(), Some(E1::A1));
        assert_eq!(iter.next(), Some(E1::B1));
        assert_eq!(iter.next(), Some(E1::C1(Default::default())));
        assert_eq!(
            iter.next(),
            Some(E1::D1 {
                s: Default::default()
            })
        );
        assert_eq!(iter.next(), None,);
    }
}
