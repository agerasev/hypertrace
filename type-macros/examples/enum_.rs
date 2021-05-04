use hypertrace_type_macros::{Named, SizedEntity};
use types::{config::HOST_CONFIG, Entity};

#[derive(Named, SizedEntity)]
enum E {
    #[getter]
    A {
        #[getter]
        x: i32,
        y: u8,
    },
    B((), usize),
    C(),
    D,
}

fn main() {
    for (key, value) in E::type_source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
