# derive_macro1

derive macro の例。

- proc_macro crate …… コンパイラが提供する proc macro 用の crate
- quote crate …… quasi-quoting で proc_macro2::TokenStream を生成する quote マクロを提供する crate
- syn crate …… proc_macro::TokenStream を構文木に parse する crate
