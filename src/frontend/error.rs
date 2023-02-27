use std::{error::Error, fmt::Display};

use super::scanner::Token;
#[derive(Debug, Clone)]
pub struct SyntaxError<'a>(&'a [Token], String);
