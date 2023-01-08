# strum

[crates:strum] の例を書く。 enum と string の変換などのマクロやトレイトを提供するクレート。

[crates:strum]: https://crates.io/crates/strum

## メモ

- `std::convert::AsRef<str>` → `strum::AsRefStr`
- `std::fmt::Display` → `strum::Display`
- `strum::EnumCount` (`X::COUNT`) → `strum::EnumCount`
- `XDiscriminants` → `strum::EnumDiscriminants`
- `strum::IntoEnumIterator` → `strum::EnumIter`
- `strum::EnumMessage` → `strum::EnumMessage`
- `strum::EnumProperty` → `strum::EnumProperty`
- `std::str::FromStr` + `std::convert::TryFrom<&str>` → `strum::EnumString`
- `strum::VariantNames` (`X::VARIANTS`) → `strum::EnumVariantNames`
- `strum::FromRepr` (`X::from_repr(usize)`) → `strum::FromRepr`
- `std::convert::From<X> for &'static str` → `strum::IntoStaticStr`
