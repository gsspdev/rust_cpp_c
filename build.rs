// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=dylib=lincoln_cpp");
    println!("cargo:rustc-link-lib=dylib=lincoln");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("liblincoln_cpp.dylib");
    std::fs::copy("liblincoln_cpp.dylib", dest_path).unwrap();
}
