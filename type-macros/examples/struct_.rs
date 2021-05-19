use hypertrace_type_macros::{Entity, EntityId, SizedEntity, EntitySource};
use types::{config::HOST_CONFIG, EntitySource};

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct A {
    #[getter]
    x: i32,
    y: (),
}

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct B;

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct C(A, #[getter] B);

fn main() {
    for (key, value) in C::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
