# derive_macro1

derive macro の例。

- proc_macro crate …… コンパイラが提供する proc macro 用の crate
- quote crate …… quasi-quoting で proc_macro2::TokenStream を生成する quote マクロを提供する crate
- syn crate …… proc_macro::TokenStream を構文木に parse する crate


## v1

derive macro helper attribute を持たないもの

<https://github.com/bouzuya/rust-examples/tree/7c3133ce43a61204f271dbe3366cca640598f5fa/derive_macro1>

## v2

derive macro helper attribute を持つもの

<https://github.com/bouzuya/rust-examples/tree/53089910a305c871a830c8d1753acd605997a756/derive_macro1>
