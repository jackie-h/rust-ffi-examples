use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    cc::Build::new()
        .cpp(true)
        .file("src/Test.cpp")
        .out_dir(format!("{}/../target/lib",manifest_dir))
        .include("include")
        .compile("cpp_examples");
}