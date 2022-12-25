use std::{path::Path, ptr::null};

use colored::Colorize;

use crate::frontend::scanner::Position;

#[derive(Debug, Clone)]
pub struct Diaganostics<'a> {
    pub context: *const Context<'a>,
}

impl<'a> Diaganostics<'a> {
    pub fn context(&mut self) -> &Context<'a> {
        unsafe { &*self.context }
    }
    pub fn file_path(&mut self) -> &str {
        self.context().file_path.to_str().unwrap()
    }
    pub fn log(&mut self, position: Option<&Position>, title: &str, msg: String) {
        let mut location: String = String::new();
        if let Some(position) = position {
            location = format!("{}:{}", position.line + 1, position.start_in_line + 1);
        }
        print!(
            "[ {} ] - {title} \n\t{} - {}",
            "rottenmangos".bold(),
            format!("{}:{location}", self.file_path()).bold().yellow(),
            msg
        )
    }
    pub fn log_wall(&mut self, title: &str, msg: &[String]) {
        println!(
            "[ {} ] - {title}\n\t{}",
            "rottenmangos".bold().black(),
            format!("{}", self.file_path().bold()).yellow()
        );
        msg.iter().for_each(|line| {
            println!("\t\t{}", line);
        });
    }
}

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub file_path: &'a Path,
    pub diagnostics: Box<Diaganostics<'a>>,
    pub flags: Flags,
}
#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub display_bytecode: bool,
}
impl<'a> Context<'a> {
    pub fn new(file_path: &'a Path, flags: Flags) -> Context {
        let mut context = Context {
            file_path,
            flags,
            diagnostics: Box::new(Diaganostics { context: null() }),
        };
        context.diagnostics.context = &mut context;
        context
    }
}
