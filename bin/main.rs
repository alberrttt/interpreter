use std::{cell::RefCell, ffi::OsString, fs::read_to_string, path::Path, rc::Rc, time::Instant};

use clap::Parser;
use limesherbet::{
    backend::vm::VirtualMachine,
    cli_helper::Diagnostics,
    common::{debug::dissasemble_chunk, interner::StringInterner, opcode::OpCode, value::Value},
    frontend::compiler::{Compiler, FunctionType},
};
mod test;
fn main() {
    #[cfg(debug_assertions)]
    {
        test::test()
    }
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    let source = read_to_string(path).unwrap();
    let interner = Rc::new(RefCell::new(StringInterner::default()));
    let diagnostics = create_rc(Diagnostics::new(path));
    let compiler = Compiler::new(interner.clone(), diagnostics.clone(), FunctionType::Script);
    let compiled = compiler.compile(source).unwrap();
    if cli.display_bytecode {
        dissasemble_chunk(&compiled.chunk, "main");
    }
    let interner = Rc::try_unwrap(interner).unwrap().into_inner();
    let mut vm = VirtualMachine::new(interner);

    vm.stack.push(Value::Void);
    vm.call(&compiled, 0);
    vm.run();
}

pub type RcRf<T> = Rc<RefCell<T>>;

pub fn create_rc<T>(v: T) -> RcRf<T> {
    Rc::new(RefCell::new(v))
}
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    path: OsString,

    #[arg(long = "dbc", help = "Displays the compiled bytecode")]
    display_bytecode: bool,
}
