use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, Arm, Expr, Pat, PatIdent};
// target use of macro
// use limesherbet_macros::native_macro;
// static array = native!{ foo => Native(), bar => Native()};
// foo!(), bar!() returns the index of the native in the array
pub fn native(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Arms);
    let mut static_array: Vec<Expr> = Vec::new();
    let binding = parsed.clone();
    let mut index_macros = Vec::new();
    binding.0.iter().enumerate().for_each(|(index, arm)| {
        let arm = arm;
        let Pat::Ident(PatIdent { ident, .. }) = arm.pat.clone() else {
            panic!()
        };
        static_array.push(*arm.body.clone());
        index_macros.push(create_macro(ident, index));
    });
    let static_array = static_array.clone();
    let len = static_array.len();

    quote! {
        pub const NATIVES_LEN: usize = #len;
        pub static NATIVES: [Native; #len] = [#(#static_array),*];
        pub mod MACROS {
            #(#index_macros)*
        }
    }
    .into()
}

fn create_macro(name: syn::Ident, index: usize) -> proc_macro2::TokenStream {
    let name = format_ident!("idx_{}", name);
    quote! {
        macro_rules! #name {
            () => {
                #index
            };
        }
        pub(crate) use #name;
    }
}

#[derive(Debug, Clone)]
struct Arms(pub Vec<Arm>);

impl Parse for Arms {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut arms = Arms(Vec::new());
        while !input.is_empty() {
            arms.0.push(input.parse()?);
        }
        Ok(arms)
    }
}
