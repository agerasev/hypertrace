use hypertrace_type_macros::{SizedEntity};
use types::{Entity, config::HOST_CONFIG};

#[derive(SizedEntity)]
struct A {
    x: i32,
    y: (),
}

#[derive(SizedEntity)]
struct B;

#[derive(SizedEntity)]
struct C(A, B);

fn main() {
    for (key, value) in C::entity_source(&HOST_CONFIG).tree.into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
