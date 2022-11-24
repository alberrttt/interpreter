use clap::Parser;
use rottenmangos::frontend::{parser::Parser as _Parser, scanner::Scanner};

fn main() {
    let cli = Cli::parse();
    let mut scanner = Scanner::new(cli.source);
    scanner.scan_thru();
    let mut parser = _Parser::new(&mut scanner);
    println!("{:#?}", parser.parse());
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_parser)]
    source: String,
}
