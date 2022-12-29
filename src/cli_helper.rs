use std::{
    cell::{Ref, RefCell},
    path::Path,
    rc::Rc,
};

use colored::Colorize;

use crate::frontend::scanner::Position;

#[derive(Debug, Clone)]
pub struct Diaganostics<'a> {
    pub context: Option<Rc<RefCell<Context<'a>>>>,
}

impl<'a> Diaganostics<'a> {
    pub fn context(&mut self) -> Ref<'_, Context<'a>> {
        self.context.as_ref().unwrap().borrow()
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
}

impl<'a> Context<'a> {
    pub fn new(file_path: &'a Path) -> Rc<RefCell<Context>> {
        let diagnostics = Diaganostics { context: None };
        let context = Rc::new(RefCell::new(Context {
            file_path,
            diagnostics: Box::new(diagnostics),
        }));
        context.borrow_mut().diagnostics.context = Some(context.clone());

        context
    }
}
