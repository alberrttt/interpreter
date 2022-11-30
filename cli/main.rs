use std::{ffi::OsString, fs::read_to_string, path::Path};

use clap::Parser;
use rottenmangos::{
    backend::vm::VM, cli_context, common::debug::dissasemble_chunk, frontend::compiler::Compiler,
};

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    let source = read_to_string(path).unwrap();

    let mut context = cli_context::Context::new(path);
    let compiler = Compiler::new(&mut context);

    let compiled = compiler.compile(source);
    dissasemble_chunk(&compiled.chunk);
    let mut vm = VM::new();

    vm.run(compiled.chunk);
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    path: OsString,
}
