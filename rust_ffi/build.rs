use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-flags=-l dylib=c++");
    } else {
        println!("cargo:rustc-flags=-l dylib=stdc++");
    }
    println!("cargo:rustc-link-search={}/../target/lib", manifest_dir);
}