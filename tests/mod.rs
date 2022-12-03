mod tests {
    use rottenmangos::{
        backend::vm::VM,
        cli_context::Context,
        frontend::compiler::{Compiler, FunctionType},
    };

    use std::fs::{self, read_to_string};

    #[test]
    pub fn test() {
        for file in fs::read_dir("./tests/scripts").unwrap() {
            let file = file.unwrap();
            let path = file.path();
            let source = read_to_string(path.clone()).unwrap();
            let mut context = Context::new(&path);
            let compiler = Compiler::new(&mut context, FunctionType::Script);

            let mut vm = VM::new();

            vm.run(compiler.compile(source).unwrap().chunk);
        }
    }
}
