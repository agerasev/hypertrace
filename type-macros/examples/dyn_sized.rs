use hypertrace_type_macros::{Entity, EntityId, EntitySource};
use types::{config::HOST_CONFIG, EntitySource};

#[derive(EntityId, Entity, EntitySource)]
struct A {
    x: i32,
    y: Vec<u8>,
}

#[derive(EntityId, Entity, EntitySource)]
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
