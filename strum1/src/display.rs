// <https://docs.rs/strum_macros/0.24.1/strum_macros/derive.Display.html>
// impl std::fmt::Display for X を追加できる

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

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
enum E2 {
    SnakeCase,
}

#[allow(dead_code)]
#[derive(strum::Display)]
#[strum(serialize_all = "shouty_snake_case")]
enum E3 {
    ShoutySnakeCase,
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
        assert_eq!(E3::ShoutySnakeCase.to_string(), "SHOUTY_SNAKE_CASE");
    }
}
