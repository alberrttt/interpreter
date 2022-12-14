use std::{ffi::OsString, fs::read_to_string, path::Path};

use clap::Parser;
use rottenmangos::{
    backend::vm::VirtualMachine,
    cli_context,
    common::value::Value,
    frontend::compiler::{Compiler, FunctionType},
};

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    let source = read_to_string(path).unwrap();

    let mut context = cli_context::Context::new(path);
    let mut vm = VirtualMachine::new();
    let compiler = Compiler::new(&mut context, FunctionType::Script);
    let compiled = compiler.compile(source).unwrap();
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
