use std::{
    env,
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};

use proc_macro::{self, TokenStream};

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
fn recurse_dir(path: &Path, stream: &mut TokenStream, post_pend: String) {
    for (i, file) in fs::read_dir(path).unwrap().enumerate() {
        let file = file.unwrap();
        if file.file_type().unwrap().is_file() {
            let path = file.path().as_path().to_string_lossy().to_string();
            let t: TokenStream = format!(
                "
#[test]
fn {}{}() {{
use rottenmangos::{{
    backend::vm::VM,
    cli_context::Context,
    frontend::compiler::{{Compiler, FunctionType}},
}};
use std::path::Path;
use std::fs::read_to_string;
let source = read_to_string(Path::new(r#\"{path}\"#)).unwrap();
let mut context = Context::new(Path::new(r#\"{path}\"#));
let compiler = Compiler::new(&mut context, FunctionType::Script);
let mut vm = VM::new();

vm.run(compiler.compile(source).unwrap().chunk);
}}\n",
                &post_pend,
                file.path().file_stem().unwrap().to_str().unwrap(),
            )
            .parse()
            .unwrap();
            stream.extend(t.into_iter());
        } else {
            let mut tmp = post_pend.clone();
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
        }
    }
}
