use std::{
    cell::RefCell, ffi::OsString, fs::read_to_string, mem::transmute, path::Path, ptr::addr_of,
    rc::Rc, time::Instant,
};

use clap::Parser;
use rottenmangos::{
    backend::vm::VirtualMachine,
    cli_context,
    common::{
        interner::StringInterner,
        value::{AsValue, Value},
    },
    frontend::compiler::{Compiler, FunctionType},
};

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    let source = read_to_string(path).unwrap();

    let mut context = cli_context::Context::new(path);
    let mut vm = VirtualMachine::new();
    let mut interner = StringInterner::new();
    let compiler = Compiler::new(
        Rc::new(RefCell::new(interner)),
        &mut context,
        FunctionType::Script,
    );
    let start = Instant::now();
    let compiled = compiler.compile(source).unwrap();
    println!(
        "took {}s to compile to bytecode",
        start.elapsed().as_secs_f64()
    );
    vm.stack.push(Value::Void);
    vm.call(compiled, 0);
    vm.run();
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    path: OsString,
}
