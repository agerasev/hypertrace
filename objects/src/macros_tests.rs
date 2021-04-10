use type_macros::SizedEntity;
use types::{Entity, SizedEntity, config::HOST_CONFIG};

#[derive(SizedEntity)]
struct Struct0 {
    pub x: u8,
    pub y: i32,
    pub z: (),
    pub w: [u8; 3],
}

#[test]
fn struct0() {
    assert_eq!(Struct0::align(&HOST_CONFIG), 4);
    assert_eq!(Struct0::type_size(&HOST_CONFIG), 12);
}
