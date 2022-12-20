use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use proc_macro::{self, TokenStream};
use quote::{format_ident, quote};
#[proc_macro]
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
                    use rottenmangos::{
                        backend::vm::VirtualMachine,
                        cli_context::Context,
                        frontend::compiler::{{Compiler, FunctionType}},
                        common::value::Value,
                    };
                    use std::path::Path;
                    use     std::fs::read_to_string;
                let source = read_to_string(Path::new(#path_string)).unwrap();
                let mut context = Context::new(Path::new(#path_string));
                let compiler = Compiler::new(&mut context, FunctionType::Script);
                let compiled = compiler.compile(source).unwrap();

                let mut vm = VirtualMachine::new();
                vm.stack.push(Value::Void);

                vm.call(compiled,0);

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