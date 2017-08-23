extern crate bindgen;

fn main() {
    let generated = bindgen::builder()
        .header("src/lib.h")
        .no_unstable_rust()
        .ctypes_prefix("libc")
        // TODO Make this less hardcoded
        .clang_arg("-I")
        .clang_arg("/usr/include/startup-notification-1.0")
        .generate().unwrap();
    generated.write_to_file("src/gen.rs").unwrap();
    // TODO Static linking feature
}
