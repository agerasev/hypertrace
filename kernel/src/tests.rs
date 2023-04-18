use std::process::Command;

fn run_bin(name: &str) {
    assert!(Command::new(format!("{}/bin/{}", env!("OUT_DIR"), name))
        .status()
        .unwrap()
        .success());
}

#[test]
fn host_kernel_test() {
    run_bin("kernel_f32");
    run_bin("kernel_f64");
}
