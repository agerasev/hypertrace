use hypertrace_type_macros::{Named, SizedEntity, Sourced};
use types::{config::HOST_CONFIG, Sourced};

#[derive(Named, SizedEntity, Sourced)]
struct A {
    #[getter]
    x: i32,
    y: (),
}

#[derive(Named, SizedEntity, Sourced)]
struct B;

#[derive(Named, SizedEntity, Sourced)]
struct C(A, B);

fn main() {
    for (key, value) in C::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
