use std::{cell::RefCell, ffi::OsString, fs::read_to_string, path::Path, rc::Rc, time::Instant};

use clap::Parser;
use macros::key_value_array;
use rottenmangos::{
    backend::vm::VirtualMachine,
    cli_context::{self, Flags},
    common::{interner::StringInterner, value::Value},
    frontend::compiler::{Compiler, FunctionType},
};

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    let source = read_to_string(path).unwrap();

    let flags = Flags {
        display_bytecode: cli.display_bytecode,
    };
    let mut context = cli_context::Context::new(path, flags);
    let interner = StringInterner::default();
    let interner_ref = Rc::new(RefCell::new(interner));
    let compiler = Compiler::new(interner_ref.clone(), &mut context, FunctionType::Script);
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
