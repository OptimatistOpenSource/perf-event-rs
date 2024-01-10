extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    let linux_headers_path = {
        let path = env::var("LINUX_HEADERS_PATH").expect("LINUX_HEADERS_PATH not present");
        println!("LINUX_HEADERS_PATH: {}", path);
        let path = Path::new(&path).canonicalize().unwrap();
        path.to_str().unwrap().to_string()
    };

    let bindings_output_path = "src/syscall/bindings/bindgen.rs";

    bindgen::Builder::default()
        .derive_default(true)
        .generate_comments(false)
        .prepend_enum_name(false)
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include", linux_headers_path))
        .generate()
        .unwrap_or_else(|e| panic!("Failed to generate bindings: {}", e))
        .write_to_file(bindings_output_path)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", bindings_output_path, e));
}
