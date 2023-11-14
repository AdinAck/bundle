use proc_macro::TokenStream;
use syn::{DeriveInput, Data};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

fn impl_bundle(body: DeriveInput) -> TokenStream2 {
    match body.data {
        Data::Enum(e) => {
            for variant in e.variants {
                for field in variant.fields {
                    println!("{}", field.ident.unwrap().to_string());
                }
            }
        }
        _ => panic!("Bundle must be an enum")
    }

    quote! { }

    // let input: TokenStream2 = input.into();

    // let bundle_data: BundleData = syn::parse2(input).unwrap();

    // let name = bundle_data.name;
    // let types = bundle_data.types.items;

    // let use_macro_name = format_ident!("use_{}", inflector::cases::snakecase::to_snake_case(&name.to_string()));
    // let match_macro_name = format_ident!("match_{}", inflector::cases::snakecase::to_snake_case(&name.to_string()));

    // let common = quote! {
    //     #[allow(non_camel_case_types)]
    //     pub enum #name {
    //         #(#types(#types)),*
    //     }

    //     #(
    //         impl Into<#name> for #types {
    //             #[inline]
    //             fn into(self) -> #name {
    //                 #name::#types(self)
    //             }
    //         }
    //     )*

    //     #[allow(unused)]
    //     macro_rules! #use_macro_name {
    //         ( $BUNDLE:expr, |$LOCAL:ident| $CODE:block ) => {
    //             match $BUNDLE {
    //                 #(
    //                     #name::#types($LOCAL) => $CODE
    //                 ),*
    //             }
    //         };
    //     }

    //     #[allow(unused)]
    //     macro_rules! #match_macro_name {
    //         ( $VALUE:expr, $TYPE:ident::$ATTR:ident => $MATCH:block else $ELSE:block ) => {
    //             match $VALUE {
    //                 #(
    //                     #types::$ATTR => {
    //                         type $TYPE = #types;
    //                         $MATCH
    //                     }
    //                 )*

    //                 _ => $ELSE
    //             }
    //         };
    //     }
    // };

    // common
}

#[proc_macro_derive(Bundle)]
pub fn bundle(input: TokenStream) -> TokenStream {
    impl_bundle(syn::parse2(input.into()).unwrap()).into()
}