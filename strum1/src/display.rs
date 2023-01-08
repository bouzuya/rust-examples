// <https://docs.rs/strum_macros/0.24.1/strum_macros/derive.Display.html>
// impl std::fmt::Display for X を追加できる

// <https://github.com/serde-rs/serde/blob/ce0844b9ecc32377b5e4545d759d385a8c46bc6a/serde_derive/src/internals/case.rs#L38-L47>

#[allow(dead_code)]
#[derive(strum::Display)]
enum E1 {
    A1,
    #[strum(serialize = "b")]
    B1,
    C1(usize),
    D1 {
        s: String,
    },
}

// strum_macros::helpers::metadata::EnumMeta
// - serialize_all
//   - {case_style}
//     - CamelCase,
//     - KebabCase,
//     - MixedCase,
//     - ShoutySnakeCase,
//     - SnakeCase,
//     - TitleCase,
//     - UpperCase,
//     - LowerCase,
//     - ScreamingKebabCase,
//     - PascalCase,
// - ascii_case_insensitive
// - crate
//   - {crate_module_path}
// - use_phf
//
// strum_macros::helpers::case_style
// impl CaseStyleHelpers for Ident
// The process transferred to heck crate <https://crates.io/crates/heck>
//
// > match case_style {
// >     CaseStyle::PascalCase => ident_string.to_upper_camel_case(),
// >     CaseStyle::KebabCase => ident_string.to_kebab_case(),
// >     CaseStyle::MixedCase => ident_string.to_lower_camel_case(),
// >     CaseStyle::ShoutySnakeCase => ident_string.to_shouty_snake_case(),
// >     CaseStyle::SnakeCase => ident_string.to_snake_case(),
// >     CaseStyle::TitleCase => ident_string.to_title_case(),
// >     CaseStyle::UpperCase => ident_string.to_uppercase(),
// >     CaseStyle::LowerCase => ident_string.to_lowercase(),
// >     CaseStyle::ScreamingKebabCase => ident_string.to_kebab_case().to_uppercase(),
// >     CaseStyle::CamelCase => {
// >         let camel_case = ident_string.to_upper_camel_case();
// >         let mut pascal = String::with_capacity(camel_case.len());
// >         let mut it = camel_case.chars();
// >         if let Some(ch) = it.next() {
// >             pascal.extend(ch.to_lowercase());
// >         }
// >         pascal.extend(it);
// >         pascal
// >     }
// > }
//
// > Ok(match text {
// >     "camel_case" | "PascalCase" => CaseStyle::PascalCase,
// >     "camelCase" => CaseStyle::CamelCase,
// >     "snake_case" | "snek_case" => CaseStyle::SnakeCase,
// >     "kebab_case" | "kebab-case" => CaseStyle::KebabCase,
// >     "SCREAMING-KEBAB-CASE" => CaseStyle::ScreamingKebabCase,
// >     "shouty_snake_case" | "shouty_snek_case" | "SCREAMING_SNAKE_CASE" => {
// >         CaseStyle::ShoutySnakeCase
// >     }
// >     "title_case" => CaseStyle::TitleCase,
// >     "mixed_case" => CaseStyle::MixedCase,
// >     "lowercase" => CaseStyle::LowerCase,
// >     "UPPERCASE" => CaseStyle::UpperCase,
// >     _ => return Err(()),
// > })

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
enum E2 {
    SnakeCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum E3 {
    ScreamingSnakeCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "camel_case")]
enum E4 {
    CamelCaseBug,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "camelCase")]
enum E5 {
    CamelCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "kebab-case")]
enum E6 {
    KebabCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "SCREAMING-KEBAB-CASE")]
enum E7 {
    ScreamingKebabCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "PascalCase")]
enum E8 {
    PascalCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "lowercase")]
enum E9 {
    LowerCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "UPPERCASE")]
enum E10 {
    UpperCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
enum E11 {
    #[strum(to_string = "to_string", serialize = "serialize")]
    ToStringOrSerialize,
    #[strum(serialize = "short", serialize = "loooong")]
    UseMaxLenSerialize,
    #[strum(serialize = "def", serialize = "abc")]
    UseLastMultipleMaxLen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e1_test() {
        assert_eq!(E1::A1.to_string(), "A1");
        assert_eq!(E1::B1.to_string(), "b");
        assert_eq!(E1::C1(1).to_string(), "C1");
        assert_eq!(
            E1::D1 {
                s: String::default()
            }
            .to_string(),
            "D1"
        );
    }

    #[test]
    fn e2_test() {
        assert_eq!(E2::SnakeCase.to_string(), "snake_case");
    }

    #[test]
    fn e3_test() {
        assert_eq!(E3::ScreamingSnakeCase.to_string(), "SCREAMING_SNAKE_CASE");
    }

    #[test]
    fn e4_test() {
        // `serialize_all = "camel_case"` -> PascalCase
        // <https://github.com/Peternator7/strum/pull/250>
        assert_eq!(E4::CamelCaseBug.to_string(), "CamelCaseBug");
    }

    #[test]
    fn e5_test() {
        assert_eq!(E5::CamelCase.to_string(), "camelCase");
    }

    #[test]
    fn e6_test() {
        assert_eq!(E6::KebabCase.to_string(), "kebab-case");
    }

    #[test]
    fn e7_test() {
        assert_eq!(E7::ScreamingKebabCase.to_string(), "SCREAMING-KEBAB-CASE");
    }

    #[test]
    fn e8_test() {
        assert_eq!(E8::PascalCase.to_string(), "PascalCase");
    }

    #[test]
    fn e9_test() {
        assert_eq!(E9::LowerCase.to_string(), "lowercase");
    }

    #[test]
    fn e10_test() {
        assert_eq!(E10::UpperCase.to_string(), "UPPERCASE");
    }

    #[test]
    fn e11_test() {
        assert_eq!(E11::ToStringOrSerialize.to_string(), "to_string");
        assert_eq!(E11::UseMaxLenSerialize.to_string(), "loooong");
        assert_eq!(E11::UseLastMultipleMaxLen.to_string(), "abc");
    }
}
