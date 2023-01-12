use proc_macro2::TokenStream;
use quote::quote;
use std::env::var;
use syn::Variant;

pub fn handle_variant(variant: &Variant) -> TokenStream {
    variant
        .attrs
        .iter()
        .for_each(|attr| println!("{:?}", attr.path));
    quote!().try_into().unwrap()
}
