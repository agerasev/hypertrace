#![allow(dead_code)]

use hypertrace_type_macros::{Entity, EntityId, SizedEntity, EntitySource};
use types::{config::HOST_CONFIG, EntitySource};

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
union A {
    #[getter]
    x: i32,
    y: (),
}

fn main() {
    for (key, value) in A::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
