use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[proc_macro_derive(VariantsFn)]
pub fn derive_variants_fn(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let data_enum = if let syn::Data::Enum(data_enum) = &input.data {
        data_enum
    } else {
        return TokenStream::from(
            syn::Error::new(input.span(), "VariantsFn can only be derived for enums")
                .to_compile_error(),
        );
    };

    let enum_ident = input.ident;
    let enum_variant_names = data_enum
        .variants
        .iter()
        .map(|variant| variant.ident.to_string());
    let output = quote::quote! {
        impl #enum_ident {
            pub fn variants() -> &'static [&'static str] {
                &[#(#enum_variant_names,)*]
            }
        }
    };

    TokenStream::from(output)
}
