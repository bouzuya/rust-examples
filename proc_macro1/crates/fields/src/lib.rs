struct Arg {
    field_name: syn::Ident,
    _eq: syn::Token![=],
    field_type: syn::Type,
}

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Arg {
            field_name: input.parse()?,
            _eq: input.parse()?,
            field_type: input.parse()?,
        })
    }
}

struct MacroInput {
    args: Vec<Arg>,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();
        while !input.is_empty() {
            let arg: Arg = input.parse()?;
            args.push(arg);

            if input.is_empty() {
                break;
            }

            let _: syn::Token![,] = input.parse()?;
        }
        Ok(MacroInput { args })
    }
}

#[proc_macro]
pub fn fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as MacroInput);
    let field_definitions = input.args.iter().map(|arg| {
        let field_name = &arg.field_name.clone();
        let field_type = &arg.field_type.clone();
        quote::quote! {
            pub #field_name: #field_type
        }
    });
    let output = quote::quote! {
        pub struct Struct1 {
            #(#field_definitions,)*
        }
    };
    output.into()
}
