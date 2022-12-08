#![feature(proc_macro_quote)]
use std::{
    env,
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};

use proc_macro::{self, TokenStream};
use quote::quote;
#[proc_macro]
pub fn make_tests(_item: TokenStream) -> TokenStream {
    let mut stream = TokenStream::new();
    println!("{:?}", env::current_dir());
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("scripts");
    recurse_dir(path.as_path(), &mut stream, String::new());
    stream
}
fn recurse_dir(path: &Path, stream: &mut TokenStream, pre_pend: String) {
    let mut test_gen = Vec::new();
    for (i, file) in fs::read_dir(path).unwrap().enumerate() {
        let file = file.unwrap();
        if file.file_type().unwrap().is_file() {
            let path = file.path();
            let name = path.file_stem().unwrap().to_str().unwrap();
            let name = pre_pend.to_owned() + name;
            let token = quote! {
                #[test]
                fn #name{
                    use rottenmangos::{
                   backend::vm::VM,
                    cli_context::Context,
                    frontend::compiler::{{Compiler, FunctionType}},
                };
                use std::path::Path;
                use std::fs::read_to_string;
                let source = read_to_string(Path::new("{path}")).unwrap();
                let mut context = Context::new(Path::new("{path}"));
                let compiler = Compiler::new(&mut context, FunctionType::Script);
                let mut vm = VM::new();

                vm.run(compiler.compile(source).unwrap().chunk);
                }
            };
            test_gen.push(token)
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
