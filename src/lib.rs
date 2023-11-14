use proc_macro::TokenStream;
use syn::{ItemEnum, Ident};
use quote::{quote, format_ident};
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn bundle(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);

    let e: ItemEnum = syn::parse2(item).expect("Bundle must be enum");

    let vis = e.vis;
    let ident = e.ident;
    let variants: Vec<Ident> = e.variants.iter().map(|v| v.ident.clone()).collect();

    let use_macro_name = format_ident!("use_{}", inflector::cases::snakecase::to_snake_case(&ident.to_string()));
    let match_macro_name = format_ident!("match_{}", inflector::cases::snakecase::to_snake_case(&ident.to_string()));
    
    quote! {
        #vis enum #ident {
            #(
                #variants(#variants)
            ),*
        }

        #(
            impl Into<#ident> for #variants {
                #[inline]
                fn into(self) -> #ident {
                    #ident::#variants(self)
                }
            }
        )*

        #[allow(unused)]
        macro_rules! #use_macro_name {
            ( $BUNDLE:expr, |$LOCAL:ident| $CODE:block ) => {
                match $BUNDLE {
                    #(
                        #ident::#variants($LOCAL) => $CODE
                    ),*
                }
            };
        }

        #[allow(unused)]
        macro_rules! #match_macro_name {
            ( $VALUE:expr, $TYPE:ident::$ATTR:ident => $MATCH:block else $ELSE:block ) => {
                match $VALUE {
                    #(
                        #variants::$ATTR => {
                            type $TYPE = #variants;
                            $MATCH
                        }
                    )*

                    _ => $ELSE
                }
            };
        }
    }.into()
}