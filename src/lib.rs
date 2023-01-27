#![warn(unsafe_code)]

use common::value::Value;
pub mod backend;
pub mod cli_helper;
pub mod common;
pub mod frontend;
extern "C" {
    pub fn sum(a: Value, b: Value) -> Value;
}
