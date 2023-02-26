use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, Ident, Token, Visibility};

struct UnitStruct {
    vis: Visibility,
    name: Ident,
}

impl Parse for UnitStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vis: Visibility = input.parse()?;
        let _: Token!(struct) = input.parse()?;
        let name = input.parse()?;
        let _: Token!(;) = input.parse()?;
        Ok(Self { vis, name })
    }
}
struct Attrs {
    path: syn::LitStr,
    author: bool,
    year: bool,
    project: bool,
}

impl Parse for Attrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let mut me = Self {
            path,
            author: false,
            project: false,
            year: false,
        };
        for _ in 0..=2 {
            let Ok(_): syn::Result<Token!(,)> = input.parse() else { return Ok(me);};
            // up to 3x, check for format args
            let Ok(ident): syn::Result<Ident> = input.parse()  else { return Ok(me);};
            match ident.to_string().as_ref() {
                "AUTHOR" => me.author = true,
                "YEAR" => me.year = true,
                "PROJECT" => me.project = true,
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        "expected fields AUTHOR, YEAR, and/or PROJECT",
                    ))
                }
            };
        }
        Ok(me)
    }
}

fn derive_license_impl(attr: Attrs, struct_def: UnitStruct) -> TokenStream {
    let type_name = &struct_def.name;
    let path = attr.path;
    let vis = &struct_def.vis;
    let mut format_arguments = vec![];
    if attr.author {
        format_arguments.push(quote!(AUTHOR = name));
    }
    if attr.year {
        format_arguments.push(quote!(YEAR = year));
    }
    if attr.project {
        format_arguments.push(quote!(PROJECT = project));
    }
    quote!(
        #vis struct #type_name;

        impl License for #type_name {
            fn notice(&self, year: u32, name: &str, project: &str) -> String {
                format!(include_str!(#path) #(, #format_arguments)*)
            }
        }
    )
    .into()
}

#[proc_macro_attribute]
pub fn impl_license(attrs: TokenStream, item: TokenStream) -> TokenStream {
    derive_license_impl(
        syn::parse(attrs).expect("parse attrs"),
        syn::parse(item).expect("parse struct def"),
    )
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn test_attrs_parse() {
        let attrs: Attrs = syn::parse_str(r#""path value", AUTHOR"#).expect("parse");
        assert_eq!(
            attrs.path.into_token_stream().to_string(),
            r#""path value""#
        );
        assert!(attrs.author);
        assert!(!attrs.project);
        assert!(!attrs.year);
    }
}
