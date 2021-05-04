use hypertrace_type_macros::{Named, SizedEntity};
use types::{config::HOST_CONFIG, Entity};

#[derive(Named, SizedEntity)]
struct A {
    #[getter]
    x: i32,
    y: (),
}

#[derive(Named, SizedEntity)]
struct B;

#[derive(Named, SizedEntity)]
struct C(A, B);

fn main() {
    for (key, value) in C::type_source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
