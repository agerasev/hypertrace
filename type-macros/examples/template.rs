use hypertrace_type_macros::{SizedEntity};
use types::{Entity, config::HOST_CONFIG};
use std::marker::PhantomData;

#[derive(SizedEntity)]
struct A<T: Copy, U: Copy, V> {
    x: T,
    y: U,
    z: PhantomData<V>,
}

fn main() {
    for (key, value) in A::<i32, u8, usize>::entity_source(&HOST_CONFIG).tree.into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
