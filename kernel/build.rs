use std::{env, fs, path::{Path, PathBuf}, collections::BTreeMap};
use walkdir::WalkDir;

fn str_text(value: &str) -> String {
    format!("r##\"{}\"##", value)
}

fn var_text(name: &str, value: &str) -> String {
    format!("pub const {}: &'static str = {};\n", name, str_text(value))
}

fn dict_text(name: &str, dict: &BTreeMap<String, String>) -> String {
    [
        format!("pub const {}: [(&'static str, &'static str); {}] = [\n", name, dict.len()),
        dict.iter().map(|(k, v)| format!("    ({}, {}),\n", str_text(k), str_text(v))).fold(String::new(), |a, b| a + &b),
        String::from("];\n"),
    ].join("")
}

fn out_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn base_path() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn kernel_src_path() -> PathBuf {
    base_path().join("ocl")
}

fn remount_path(path: &Path, old_base: &Path, new_base: &Path) -> PathBuf {
    new_base.join(path.strip_prefix(old_base).unwrap())
}

fn write_test_defs() {
    fs::write(
        &out_path().join("test_defs.rs"),
        [
            var_text("TARGET", &env::var("TARGET").unwrap()),
        ].join("")
    ).unwrap();
}

fn embed_kernel_source() {
    let mut dict = BTreeMap::new();
    let src_path = kernel_src_path();
    let dst_path = PathBuf::from("/src/kernel");
    for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let r = dict.insert(
                format!("{}", remount_path(entry.path(), &src_path, &dst_path).display()),
                fs::read_to_string(entry.path()).unwrap(),
            );
            assert!(r.is_none());
        }
    }
    fs::write(
        &out_path().join("kernel_source.rs"),
        dict_text("FILES_STATIC", &dict),
    ).unwrap();
}

fn rerun_if_source_changed() {
    let base_path = base_path();
    let src_path = kernel_src_path();
    for entry in WalkDir::new(&src_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            println!("cargo:rerun-if-changed={}", entry.path().strip_prefix(&base_path).unwrap().display());
        }
    }
}

fn main() {
    embed_kernel_source();
    write_test_defs();

    println!("cargo:rerun-if-changed=build.rs");
    rerun_if_source_changed();
}
