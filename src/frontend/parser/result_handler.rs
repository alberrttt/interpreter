use super::Parser;

pub static mut RESULT_HANDLER: ResultHandler = ResultHandler { parser: None };
pub struct ResultHandler<'a> {
    pub parser: Option<*mut Parser<'a>>,
}
