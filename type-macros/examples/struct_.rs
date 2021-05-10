use hypertrace_type_macros::{Entity, Named, SizedEntity, Sourced};
use types::{config::HOST_CONFIG, Sourced};

#[derive(Named, Entity, SizedEntity, Sourced)]
struct A {
    #[getter]
    x: i32,
    y: (),
}

#[derive(Named, Entity, SizedEntity, Sourced)]
struct B;

#[derive(Named, Entity, SizedEntity, Sourced)]
struct C(A, #[getter] B);

fn main() {
    for (key, value) in C::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
