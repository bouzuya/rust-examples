// <https://docs.rs/strum/0.24.1/strum/derive.IntoStaticStr.html>

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::IntoStaticStr)]
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
        let a1_str: &'static str = E1::A1.into();
        assert_eq!(a1_str, "A1");
        let b1_str: &'static str = E1::B1.into();
        assert_eq!(b1_str, "B1");
        let c1_str: &'static str = E1::C1.into();
        assert_eq!(c1_str, "C1");
    }
}
