use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, token::Paren, Field, FieldMutability, Fields, FieldsUnnamed, Ident,
    ItemEnum, Type, Visibility,
};

/// Transform attached enum into a "bundle".
///
/// *What is a bundle?*
///
/// Bundles are used to accomplish dynamic dispatch in resource constrained systems (no_std).
/// A bundle can hold a finite number of types that implement a common trait.
/// The size of the bundle is known at compile time and equal to the size of the largest type in the bundle.
/// Bundles are useful for type-erasure when transporting multiple types heterogeneously.
#[proc_macro_attribute]
pub fn bundle(attr: TokenStream, item: TokenStream) -> TokenStream {
    // capture args and enum body
    let args: Option<Ident> = syn::parse2(TokenStream2::from(attr)).ok();
    let item = TokenStream2::from(item);

    let export = args
        .and_then(|arg| Some(arg.to_string() == "export"))
        .and_then(|present| present.then_some(quote! { #[macro_export] }));

    // parse enum body
    let mut e: ItemEnum = syn::parse2(item).expect("Bundle must be an enum.");

    assert!(
        !e.variants.is_empty(),
        "Bundle must contain at least one variant."
    );

    // transform into proper tuple variants
    e.variants = e
        .variants
        .iter()
        .map(|v| {
            let mut v = v.clone();

            match v.fields {
                Fields::Unit => {
                    let ident = v.ident.clone();

                    let mut punc = Punctuated::new();
                    punc.push(Field {
                        attrs: Vec::new(),
                        vis: Visibility::Inherited,
                        mutability: FieldMutability::None,
                        ident: None,
                        colon_token: None,
                        ty: Type::Verbatim(quote! { #ident }),
                    });

                    v.fields = Fields::Unnamed(FieldsUnnamed {
                        paren_token: Paren::default(),
                        unnamed: punc,
                    });

                    v
                }
                Fields::Unnamed(_) => v,
                Fields::Named(_) => panic!("Bundles cannot contain struct variants."),
            }
        })
        .collect();

    // extract visibility, ident, variant idents/types, and generics for generation
    let ident = e.ident.clone();
    let (impl_generics, ty_generics, where_clause) = e.generics.split_for_impl().clone();
    let variant_idents: Vec<Ident> = e.variants.iter().cloned().map(|v| v.ident).collect();
    let variant_tys: Vec<Type> = e
        .variants
        .iter()
        .cloned()
        .map(|v| match v.fields {
            Fields::Unnamed(fields) => fields.unnamed.first().unwrap().ty.clone(),
            _ => {
                unreachable!("All variant fields are unnamed by now.")
            }
        })
        .collect();

    // validate variants hold eactly one type
    for variant in &e.variants {
        if variant.fields.len() != 1 {
            panic!("Bundle variants must hold exacly one type.")
        }
    }

    let use_macro_name = format_ident!(
        "use_{}",
        inflector::cases::snakecase::to_snake_case(&ident.to_string())
    );

    quote! {
        #e

        // Into's for each variant of the bundle
        #(
            impl #impl_generics Into<#ident #ty_generics> for #variant_tys #where_clause {
                #[inline]
                fn into(self) -> #ident #ty_generics {
                    #ident::#variant_idents(self)
                }
            }
        )*

        // use macro for dispatch
        #[allow(unused)]
        #export
        macro_rules! #use_macro_name {
            ( $BUNDLE:expr, |$LOCAL:ident| $CODE:block ) => {
                match $BUNDLE {
                    #(
                        #ident::#variant_idents($LOCAL) => $CODE
                    ),*
                }
            };
        }
    }
    .into()
}
