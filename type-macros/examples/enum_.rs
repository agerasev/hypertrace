use hypertrace_type_macros::{SizedEntity};
use types::{Entity, config::HOST_CONFIG};

#[derive(SizedEntity)]
enum E {
    A { x: i32, y: u8 },
    B((), usize),
    C(),
    D,
}

/*
#[derive(SizedEntity)]
enum G {
    E(E),
    F(F),
}
*/
fn main() {
    for (key, value) in E::entity_source(&HOST_CONFIG).tree.into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
