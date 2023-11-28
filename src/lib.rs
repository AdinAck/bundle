use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, ItemEnum, Variant};

#[proc_macro_attribute]
pub fn bundle(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = TokenStream2::from(attr);
    let item = TokenStream2::from(item);

    // for now there is only one possible optional argument: "export"
    let arg: Option<Ident> = syn::parse2(args).ok();

    let export = arg.as_ref().map(|arg| {
        let s = arg.to_string();
        if s != "export" {
            panic!("Unexpected argument \"{}\"", s);
        }

        quote! { #[macro_export] }
    });

    let e: ItemEnum = syn::parse2(item).expect("Bundle must be enum");

    let vis = e.vis;
    let ident = e.ident;
    let variants: Vec<Variant> = e.variants.iter().cloned().collect();
    let variant_idents: Vec<Ident> = e.variants.iter().cloned().map(|v| v.ident).collect();

    for variant in &e.variants {
        if variant.fields.len() != 1 {
            panic!("Bundle variants must hold corresponding type.")
        }
    }

    let use_macro_name = format_ident!(
        "use_{}",
        inflector::cases::snakecase::to_snake_case(&ident.to_string())
    );
    let match_macro_name = format_ident!(
        "match_{}",
        inflector::cases::snakecase::to_snake_case(&ident.to_string())
    );

    quote! {
        #vis enum #ident {
            #(
                #variants
            ),*
        }

        #(
            impl Into<#ident> for #variant_idents {
                #[inline]
                fn into(self) -> #ident {
                    #ident::#variant_idents(self)
                }
            }
        )*

        #export
        #[allow(unused)]
        macro_rules! #use_macro_name {
            ( $BUNDLE:expr, |$LOCAL:ident| $CODE:block ) => {
                match $BUNDLE {
                    #(
                        #ident::#variant_idents($LOCAL) => $CODE
                    ),*
                }
            };
        }

        #export
        #[allow(unused)]
        macro_rules! #match_macro_name {
            ( $VALUE:expr, $TYPE:ident::$ATTR:ident => $MATCH:block else $ELSE:block ) => {
                match $VALUE {
                    #(
                        #variants::$ATTR => {
                            type $TYPE = #variant_idents;
                            $MATCH
                        }
                    )*

                    _ => $ELSE
                }
            };
        }
    }
    .into()
}
