use hypertrace_type_macros::{Entity, EntityId, SizedEntity, EntitySource};
use types::{config::HOST_CONFIG, EntitySource};

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
enum E {
    A {
        #[getter]
        x: i32,
        y: u8,
    },
    #[getter]
    B((), #[getter] usize),
    C(),
    D,
}

fn main() {
    for (key, value) in E::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
