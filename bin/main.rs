use std::{cell::RefCell, ffi::OsString, fs::read_to_string, path::Path, rc::Rc, time::Instant};

use clap::Parser;
use limesherbet::{
    backend::vm::VirtualMachine,
    cli_helper::Diagnostics,
    common::{
        closure::Closure, debug::dissasemble_chunk,
        value::Value,
    },
    frontend::compiler::{Compiler, FunctionType},
};
pub fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    let source = read_to_string(path).unwrap();
    let diagnostics = create_rc(Diagnostics::new(path));
    let duration = Instant::now();
    let compiler = Compiler::new(diagnostics.clone(), FunctionType::Script);
    let elapsed = duration.elapsed();
    let (compiled, file_node) = compiler.compile(source).unwrap();

    println!("Compiled in {}s", elapsed.as_secs_f64() * 1000.0);
    if cli.display_bytecode {
        dissasemble_chunk(&compiled.chunk, "main");
    }
    if cli.display_ast {
        file_node.nodes.iter().for_each(|node| {
            println!("{node:?}");
        })
    }

    let mut vm = VirtualMachine::new();

    let mut closure = Closure {
        func: Rc::new(compiled),
        upvalues: Vec::new(),
    };
    vm.stack.push(Value::Void);
    vm.call(&mut closure, 0);
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
    #[arg(long = "dast", help = "Displays the ast")]
    display_ast: bool,
}
