mod defs;

use std::process::Command;

fn run_bin(name: &str) {
    assert!(Command::new(format!("{}/bin/{}", env!("OUT_DIR"), name)).status().unwrap().success());
}

#[test]
fn host_kernel_test() {
    let mut cfg = cmake::Config::new(env!("CARGO_MANIFEST_DIR"));
    cfg.host(defs::TARGET);
    cfg.target(defs::TARGET);
    cfg.profile("Debug");
    cfg.build_arg(format!("-j{}", num_cpus::get()));
    cfg.build();

    run_bin("kernel_f32");
    run_bin("kernel_f64");
}
