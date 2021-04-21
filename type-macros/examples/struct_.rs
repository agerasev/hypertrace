use hy_type_macros::{SizedEntity};

#[derive(SizedEntity)]
struct A {
    x: i32,
    y: (),
}

fn main() {}
