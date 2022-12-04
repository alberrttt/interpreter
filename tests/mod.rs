mod tests {
    use rottenmangos::{
        backend::vm::VM,
        cli_context::Context,
        frontend::compiler::{Compiler, FunctionType},
    };

    use std::{
        ffi::OsStr,
        fs::{self, read_to_string, FileType},
        os::windows::fs::FileTypeExt,
    };

    #[test]
    pub fn test() {
        run_dir("./tests/scripts".to_string())
    }
    pub fn run_dir(path: String) {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();
            let file_type = file.file_type().unwrap();
            let path = file.path();

            if file_type.is_dir() {
                run_dir(path.as_os_str().to_string_lossy().to_string())
            } else if file_type.is_file() && path.extension().unwrap_or(OsStr::new("")) == "mng" {
                let source = read_to_string(path.clone()).unwrap();
                let mut context = Context::new(&path);
                let compiler = Compiler::new(&mut context, FunctionType::Script);
                let mut vm = VM::new();

                vm.run(compiler.compile(source).unwrap().chunk);
            }
        }
    }
}
