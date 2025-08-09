#[proc_macro]
pub fn function_like(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);
    let output = quote::quote! {
        #input
    };
    output.into()
}
