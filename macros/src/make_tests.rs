use std::{
    fs,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
pub fn make_tests(_item: TokenStream) -> TokenStream {
    let mut stream = Vec::new();
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("scripts");
    recurse_dir(path.as_path(), &mut stream, String::new());
    let iter = stream.into_iter();
    let tmp = TokenStream::from_iter(iter);
    tmp
}
fn recurse_dir(path: &Path, stream: &mut Vec<TokenStream>, pre_pend: String) {
    for (_i, file) in fs::read_dir(path).unwrap().enumerate() {
        let file = file.unwrap();
        if file.file_type().unwrap().is_file() {
            let path = file.path();
            let path_string = path.to_str().unwrap();
            let name = path.file_stem().unwrap().to_str().unwrap();
            let name = pre_pend.to_owned() + name;
            let tmp_name = format_ident!("{}", name);
            let token = quote! {
                #[test]
                fn #tmp_name() {
                    use limesherbet::{
                        backend::vm::VirtualMachine,
                        cli_helper::{Diagnostics},
                        frontend::compiler::{{Compiler, FunctionType}},
                        common::{value::Value, interner::StringInterner},
                    };
                    use std::rc::Rc;
                    use std::cell::RefCell;

                    use std::path::Path;
                    use std::fs::read_to_string;

                    let interner = StringInterner::default();
                    let source = read_to_string(Path::new(#path_string)).unwrap();
                    let mut diagnostics = Rc::new(RefCell::new(Diagnostics::new(Path::new(#path_string))));
                    let interner_ref = Rc::new(RefCell::new(interner));
                    let compiler = Compiler::new(interner_ref.clone(), diagnostics, FunctionType::Script);
                    let compiled = compiler.compile(source).unwrap();

                    let interner = Rc::try_unwrap(interner_ref).unwrap().into_inner();
                    let mut vm = VirtualMachine::new(interner);
                    vm.stack.push(Value::Void);
                    vm.call(&compiled,0);
                    vm.run();
                }
            };
            stream.push(token.into())
        } else {
            let mut tmp = pre_pend.clone();
            tmp.push_str(
                &(file
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
                    + "_"),
            );
            recurse_dir(file.path().as_path(), stream, tmp);
        };
    }
}
