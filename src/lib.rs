#![deny(unsafe_code)]
pub mod backend;
pub mod cli_helper;
pub mod common;
pub mod frontend;
pub mod prelude;
pub mod rust_bindings;
pub(crate) mod macros {
    #[macro_export]
    macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[{}:{}]: {}", file!(), line!(), format_args!($($arg)*));
    }
}
}
