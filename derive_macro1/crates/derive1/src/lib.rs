use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[proc_macro_derive(VariantsFn, attributes(rename))]
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
    let enum_variant_names = match data_enum
        .variants
        .iter()
        .map(|variant| {
            match variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("rename"))
            {
                None => Ok(variant.ident.to_string()),
                Some(attr) => {
                    let meta_name_value = attr.meta.require_name_value().map_err(|_| {
                        syn::Error::new(attr.span(), "expected `#[rename = \"name\"]` attribute")
                    })?;
                    match &meta_name_value.value {
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) => Ok(lit_str.value()),
                        _ => Err(syn::Error::new(
                            meta_name_value.span(),
                            "expected string literal for rename value",
                        )),
                    }
                }
            }
        })
        .collect::<syn::Result<Vec<String>>>()
    {
        Ok(enum_variant_names) => enum_variant_names,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };
    let output = quote::quote! {
        impl #enum_ident {
            pub fn variants() -> &'static [&'static str] {
                &[#(#enum_variant_names,)*]
            }
        }
    };

    TokenStream::from(output)
}
