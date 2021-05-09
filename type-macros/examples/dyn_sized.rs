use hypertrace_type_macros::{Named, Entity, Sourced};
use types::{config::HOST_CONFIG, Sourced};

#[derive(Named, Entity, Sourced)]
struct A {
    x: i32,
    y: Vec<u8>,
}

#[derive(Named, Entity, Sourced)]
enum E {
    A(A),
    B((), Vec<i32>),
    C(()),
    D,
}

fn main() {
    for (key, value) in E::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
