#[proc_macro]
pub fn fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as MacroInput);
    format_macro_output(input)
}

struct MacroInput {
    args: Vec<(syn::Ident, syn::Type)>,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();
        while !input.is_empty() {
            let field_name: syn::Ident = input.parse()?;
            let _eq: syn::Token![=] = input.parse()?;
            let field_type: syn::Type = input.parse()?;
            args.push((field_name, field_type));

            if input.is_empty() {
                break;
            }

            let _: syn::Token![,] = input.parse()?;
        }
        Ok(MacroInput { args })
    }
}

fn format_macro_output(input: MacroInput) -> proc_macro::TokenStream {
    let mut fields = Vec::new();
    for (field_name, field_type) in input.args {
        fields.push(quote::quote! {
            #field_name: #field_type,
        });
    }

    let output = quote::quote! {
        struct Struct1 {
            #(#fields)*
        }
    };
    proc_macro::TokenStream::from(output)
}
