// <https://docs.rs/strum/0.24.1/strum/derive.FromRepr.html>

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::FromRepr)]
enum E1 {
    A1,
    B1,
    C1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1::from_repr(0), Some(E1::A1));
        assert_eq!(E1::from_repr(1), Some(E1::B1));
        assert_eq!(E1::from_repr(2), Some(E1::C1));
        assert_eq!(E1::from_repr(3), None);
    }
}
