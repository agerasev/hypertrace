use hypertrace_type_macros::{Entity, EntityId, EntitySource, SizedEntity};
use types::{config::HOST_CONFIG, EntitySource, SizedEntity};

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct A<T>
where
    i32: SizedEntity,
{
    t: T,
    i: i32,
}

fn main() {
    for (key, value) in A::<()>::source(&HOST_CONFIG).into_iter() {
        println!("{:?}:\n{}\n", key, value);
    }
}
