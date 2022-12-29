use std::{cell::RefCell, ffi::OsString, fs::read_to_string, path::Path, rc::Rc, time::Instant};

use clap::Parser;
use rottenmangos::{
    backend::vm::VirtualMachine,
    cli_helper::{self},
    common::{interner::StringInterner, value::Value},
    frontend::compiler::{Compiler, FunctionType},
};

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    let source = read_to_string(path).unwrap();

    let mut context = cli_helper::Context::new(path);
    let interner = StringInterner::default();
    let interner_ref = Rc::new(RefCell::new(interner));
    let compiler = Compiler::new(interner_ref.clone(), context, FunctionType::Script);
    let start = Instant::now();
    let compiled = compiler.compile(source).unwrap();
    println!(
        "took {}s to compile to bytecode",
        start.elapsed().as_secs_f64()
    );
    let interner = Rc::try_unwrap(interner_ref).unwrap().into_inner();
    let mut vm = VirtualMachine::new(interner);

    vm.stack.push(Value::Void);
    vm.call(&compiled, 0);
    vm.run();
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    path: OsString,

    #[arg(long = "dbc", help = "Displays the compiled bytecode")]
    display_bytecode: bool,
}
