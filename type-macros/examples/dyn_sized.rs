use hypertrace_type_macros::{Named, DynSizedEntity, Sourced};
use types::{config::HOST_CONFIG, Sourced};

#[derive(Named, DynSizedEntity, Sourced)]
struct A {
    x: i32,
    y: Vec<u8>,
}

fn main() {
    for (key, value) in A::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
