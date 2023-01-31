use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Arm, Expr, Macro, Pat, PatIdent};
// target use of macro
// use limesherbet_macros::native_macro;
// static array = native!{ foo => Native(), bar => Native()};
// foo!(), bar!() returns the index of the native in the array
pub fn native(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Arms);
    let mut static_array: Vec<Expr> = Vec::new();
    let len = static_array.len();
    let binding = parsed.clone();
    let index_macros = binding.0.iter().enumerate().map(|(index, arm)| {
        let arm = arm;
        let Pat::Ident(PatIdent { ident, .. }) = arm.pat.clone() else {
            panic!()
        };
        return create_macro(ident, index);
    });
    quote! {
        pub static NATIVES: [Native; #len] = [#(#static_array),*];
        #(#index_macros)*
    }
    .into()
}

fn create_macro(name: syn::Ident, index: usize) -> proc_macro2::TokenStream {
    quote! {
        macro_rules!  {
            () => {
                #index
            };
        }

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
