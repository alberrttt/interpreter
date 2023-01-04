use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Arm, Pat};

struct Arms {
    arms: Vec<Arm>,
}
// the ground work for adding a better "api" for manipulating opcodes
mod opcode;
use opcode::expand_opcode as _expand_opcode;
#[proc_macro_derive(ExpandOpCode)]
pub fn expand_opcode(input: TokenStream) -> TokenStream {
    _expand_opcode(input)
}

#[proc_macro]
pub fn key_value_array(input: TokenStream) -> TokenStream {
    let arms = parse_macro_input!(input as Arms);
    arms.arms.iter().for_each(|f| {
        println!(
            "{:?}",
            if let Pat::Lit(literal) = &f.pat {
                literal
            } else {
                panic!()
            }
        );
    });
    quote! {}.into()
}
impl Parse for Arms {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut arms = Vec::new();
        while !input.is_empty() {
            arms.push(match input.call(Arm::parse) {
                Ok(it) => it,
                Err(err) => return Err(err),
            })
        }
        Ok(Arms { arms })
    }
}
mod make_tests;
use make_tests::make_tests as _make_tests;
#[proc_macro]
pub fn make_tests(_item: TokenStream) -> TokenStream {
    _make_tests(_item)
}
