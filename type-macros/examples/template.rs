use hypertrace_type_macros::{Entity, Named, SizedEntity, Sourced};
use std::marker::PhantomData;
use types::{config::HOST_CONFIG, Sourced};

#[derive(Named, Entity, SizedEntity, Sourced)]
struct A<T: Copy + 'static, U: Copy + 'static, V: 'static> {
    #[getter]
    x: T,
    y: U,
    z: PhantomData<V>,
}

fn main() {
    for (key, value) in A::<i32, u8, usize>::source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
