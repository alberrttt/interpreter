use proc_macro::{self, TokenStream};

// the ground work for adding a better "api" for manipulating opcodes
mod opcode;
use opcode::expand_opcode as _expand_opcode;
#[proc_macro_derive(ExpandOpCode, attributes(stack, no_impl))]
pub fn expand_opcode(input: TokenStream) -> TokenStream {
    _expand_opcode(input)
}

mod make_tests;
use make_tests::make_tests as _make_tests;
#[proc_macro]
pub fn make_tests(_item: TokenStream) -> TokenStream {
    _make_tests(_item)
}

mod lookup;
use lookup::lookup as _lookup;
#[proc_macro]
pub fn lookup(input: TokenStream) -> TokenStream {
    _lookup(input)
}

mod native;
use native::native;

#[proc_macro]
pub fn native_macro(input: TokenStream) -> TokenStream {
    native(input)
}
