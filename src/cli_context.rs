use std::{path::Path, ptr::null};

use colored::Colorize;

#[derive(Debug)]
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
    pub fn log_line(&mut self, title: &str, msg: String) {
        println!(
            "[ {} ] - {title} \n\t{} - {}",
            "rottenmangos".bold(),
            format!("{}", self.file_path()).bold().yellow(),
            msg
        )
    }
    pub fn log_wall(&mut self, title: &str, msg: String) {
        println!(
            "[ {} ] - {title}\n\t{}",
            "rottenmangos".bold().black(),
            format!("{}", self.file_path().bold()).yellow()
        );
        msg.split('\n').for_each(|line| {
            println!("\t\t{}", line);
        });
    }
}

#[derive(Debug)]
pub struct Context<'a> {
    pub file_path: &'a Path,
    pub diagnostics: Box<Diaganostics<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(file_path: &'a Path) -> Context {
        let mut context = Context {
            file_path,
            diagnostics: Box::new(Diaganostics { context: null() }),
        };
        context.diagnostics.context = &mut context;
        context
    }
}
