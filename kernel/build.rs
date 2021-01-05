use std::env;
use std::fs;
use std::path::Path;

fn def(name: &str, value: &str) -> String {
    format!("pub const {}: &str = \"{}\";\n", name, value)
}

fn main() {
    fs::write(
        &Path::new(&env::var("OUT_DIR").unwrap()).join("defs.rs"),
        vec![
            def("TARGET", &env::var("TARGET").unwrap()),
        ].join("")
    ).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
