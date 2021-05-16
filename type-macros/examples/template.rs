use hypertrace_type_macros::{Entity, EntityId, SizedEntity, EntitySource};
use std::marker::PhantomData;
use types::{config::HOST_CONFIG, EntitySource};

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct A<T: Copy + 'static, U: Copy + 'static, V: 'static> {
    #[getter]
    x: T,
    y: U,
    z: PhantomData<V>,
}

fn main() {
    for (key, value) in A::<i32, u8, usize>::data_source(&HOST_CONFIG).into_iter() {
        println!("'{}':\n{}\n", key, value);
    }
}
