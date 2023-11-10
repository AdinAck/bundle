use proc_macro::TokenStream;
use syn::{
    parse::Parse,
    Ident,
    braced,
    Token
};
use quote::{quote, format_ident};
use proc_macro2::TokenStream as TokenStream2;

#[derive(Debug)]
struct Set<T> {
    items: Vec<T>,
}

impl<T> Parse for Set<T>
where
    T: Parse + PartialEq,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();

        let content;
        braced!(content in input);

        while !content.is_empty() {
            let item: T = content.parse()?;
            if items.contains(&item) {
                panic!("Identifiers must be unique.")
            }
            items.push(item);

            if content.is_empty() {
                break;
            }

            content.parse::<Token![,]>()?;
        }

        Ok(Set { items })
    }
}

struct BundleData {
    name: Ident,
    types: Set<Ident>
}

impl Parse for BundleData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let types: Set<Ident> = input.parse()?;

        Ok(Self { name, types })
    }
}

#[proc_macro]
pub fn bundle(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();

    let bundle_data: BundleData = syn::parse2(input).unwrap();

    let name = bundle_data.name;
    let types = bundle_data.types.items;

    let use_macro_name = format_ident!("use_{}", inflector::cases::snakecase::to_snake_case(&name.to_string()));
    let match_macro_name = format_ident!("match_{}", inflector::cases::snakecase::to_snake_case(&name.to_string()));

    let common = quote! {
        #[allow(non_camel_case_types)]
        pub enum #name {
            #(#types(#types)),*
        }

        #(
            impl Into<#name> for #types {
                #[inline]
                fn into(self) -> #name {
                    #name::#types(self)
                }
            }
        )*

        #[allow(unused)]
        macro_rules! #use_macro_name {
            ( $BUNDLE:expr, |$IDENT:ident| $CODE:block ) => {
                match $BUNDLE {
                    #(
                        #name::#types($IDENT) => $CODE
                    ),*
                }
            };
        }

        #[allow(unused)]
        macro_rules! #match_macro_name {
            ( $VALUE:expr, $TYPE:ident::$ATTR:ident => $MATCH:block else $ELSE:block ) => {
                match $VALUE {
                    #(
                        #types::ATTR => {
                            type $TYPE = #types;
                            $MATCH
                        }
                    )*

                    _ => $ELSE
                }
            };
        }
    };

    common.into()
}