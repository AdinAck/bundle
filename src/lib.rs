use proc_macro::TokenStream;
use syn::{
    parse::Parse,
    Ident,
    braced,
    Token, Generics
};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

// can't believe i have to do this
#[derive(Debug)]
struct TypeWithGenerics {
    ident: Ident,
    generics: Vec<Ident>
}

impl TypeWithGenerics {
    fn as_stream(&self) -> TokenStream2 {
        let ident = &self.ident;
        let generics = &self.generics;

        quote! (
            #ident<#(#generics),*>
        )
    }
}

impl Parse for TypeWithGenerics {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let generics = input.parse::<Generics>()?.type_params().map(|t| t.ident.clone()).collect();

        Ok(Self { ident, generics })
    }
}

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
    trait_type: TypeWithGenerics,
    types: Set<Ident>
}

impl Parse for BundleData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        input.parse::<Token![<]>()?;

        let trait_ident: TypeWithGenerics = input.parse()?;

        input.parse::<Token![<]>()?;

        let types: Set<Ident> = input.parse()?;

        Ok(Self { name, trait_type: trait_ident, types })
    }
}

#[proc_macro]
pub fn bundle(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();

    let bundle_data: BundleData = syn::parse2(input).unwrap();

    let name = bundle_data.name;
    let types = bundle_data.types.items;
    let trait_type = bundle_data.trait_type.as_stream();

    quote! {
        pub enum #name {
            #(#types),*
        }

        impl #name {
            fn with<F, T>(&mut self, closure: F) -> T
            where
                F: FnMut(&mut dyn #trait_type) -> T
            {
                match self {
                    #(
                        #name::#types(value) => closure(value)
                    ),*
                }
            }
        }
    }.into()
}