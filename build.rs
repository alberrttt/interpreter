use std::{env, path::Path};

use cbindgen::Language;

pub fn main() {
    println!("cargo:rerun-if-changed=c");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.h");
    cc::Build::new().file("./c/sum.c").compile("sum");
}
