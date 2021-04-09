use type_macros::Entity;
use types::{Entity, config::HOST_CONFIG};

#[derive(Entity)]
struct Struct0 {
    pub x: u32,
    pub y: [u8; 3],
}

#[test]
fn size() {
    assert_eq!(Struct0::align(&HOST_CONFIG), 4);
}
