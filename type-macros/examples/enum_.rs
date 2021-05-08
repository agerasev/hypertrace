use hypertrace_type_macros::{Named, SizedEntity, Sourced};
use types::{config::HOST_CONFIG, Sourced};

#[derive(Named, SizedEntity, Sourced)]
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
