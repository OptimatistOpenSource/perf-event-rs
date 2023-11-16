extern crate bindgen;

fn main() {
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        .header("wrapper.h")
        .clang_arg("-I/usr/include") // This may differ between distros
        .generate()
        .unwrap_or_else(|e| panic!("Failed to generate bindings: {}", e));

    let path = "src/syscall/bindings.rs";
    bindings
        .write_to_file(path)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", path, e));
}
