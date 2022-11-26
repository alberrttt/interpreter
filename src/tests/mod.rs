macro_rules! test {
    ($ident:ident,$path:literal) => {
        #[test]
        pub fn $ident() {
            let source = read_to_string(String::from("./src/tests") + $path).unwrap();
            let mut context = Context::new(Path::new($path));
            let mut compiler = Compiler::new(&mut context);

            let mut vm = VM::new();

            vm.run(compiler.compile(source).chunk);
        }
    };
}
#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use crate::{backend::vm::VM, cli_context::Context, frontend::compiler::Compiler};

    test!(add, "/scripts/add");
    test!(mul, "/scripts/multiply");
    test!(div, "/scripts/divide");
    test!(sub, "/scripts/subtract");
}
