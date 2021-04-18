use hypertrace_type_macros::{SizedEntity};

#[derive(SizedEntity)]
enum E {
    A { x: i32, y: () },
    B(usize, u8),
}

fn main() {}
