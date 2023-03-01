use std::path::Path;

use colored::Colorize;

use crate::frontend::scanner::Position;

#[derive(Debug, Clone)]
pub struct Diagnostics<'a> {
    pub file_path: &'a Path,
}
impl Default for Diagnostics<'_> {
    fn default() -> Self {
        Self {
            file_path: Path::new(""),
        }
    }
}
impl<'a> Diagnostics<'a> {
    pub fn new(path: &'a Path) -> Self {
        Diagnostics { file_path: path }
    }

    pub fn file_path(&self) -> &str {
        self.file_path.to_str().unwrap()
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
            println!("\t\t{line}");
        });
    }
}
