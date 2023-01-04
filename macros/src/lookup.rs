use std::{collections::HashMap, error::Error};

use proc_macro2::Ident;
use quote::{quote, spanned::Spanned, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, token::Colon, Arm, Expr, ExprLit, LifetimeDef, Lit, Pat,
    PatLit, TypePath,
};
pub fn lookup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let arms = parse_macro_input!(input as Definitions);
    let mut indexes = 0;
    let mut lookup_indexes: HashMap<String, usize> = HashMap::new();
    let mut lookup_array: Vec<Expr> = Vec::new();
    quote! {}.into()
}

pub struct Definitions(pub Vec<Definition>);
impl Parse for Definitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut definitions = Vec::new();
        while !input.is_empty() {
            definitions.push(input.parse().unwrap());
            if !input.is_empty() {
                input.call(syn::token::Comma::parse)?;
            }
        }
        Ok(Definitions(definitions))
    }
}
pub struct Definition {
    pub ident: Ident,
    pub colon: Colon,
    pub var_type: TypePath,
    pub equal: syn::token::Eq,
    pub initializer: Expr,
}
impl Parse for Definition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Definition {
            ident: input.parse()?,
            colon: input.parse()?,
            var_type: input.parse()?,
            equal: input.parse()?,
            initializer: input.parse()?,
        })
    }
}
