// <https://docs.rs/strum/0.24.1/strum/derive.EnumDiscriminants.html>
// variant 名だけの enum を別途定義する

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumDiscriminants)]
enum E1 {
    A1,
    B1,
    C1(usize),
    D1 { s: String },
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(name(E2D))]
enum E2 {
    A1,
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::EnumString))]
enum E3 {
    A1,
    B1,
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::EnumMessage))]
enum E4 {
    #[strum_discriminants(strum(message = "message1"))]
    A1,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use strum::EnumMessage;

    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1Discriminants::A1, E1Discriminants::from(E1::A1));
        assert_eq!(E1Discriminants::A1, E1::A1.into());
        assert_eq!(E1Discriminants::B1, E1::B1.into());
        assert_eq!(E1Discriminants::C1, E1::C1(1).into());
        assert_eq!(
            E1Discriminants::D1,
            E1::D1 {
                s: Default::default()
            }
            .into()
        );
    }

    #[test]
    fn e2_test() {
        assert_eq!(E2D::A1, E2::A1.into());
    }

    #[test]
    fn e3_test() -> anyhow::Result<()> {
        assert_eq!(E3Discriminants::A1, E3Discriminants::from_str("A1")?);
        assert_eq!(E3Discriminants::B1, E3Discriminants::from_str("B1")?);
        Ok(())
    }

    #[test]
    fn e4_test() {
        assert_eq!(E4Discriminants::A1.get_message(), Some("message1"));
    }
}
