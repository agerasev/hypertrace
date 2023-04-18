use type_macros::{Entity, EntityId, EntitySource, SizedEntity};
use types::{
    config::{self, Config},
    Entity, SizedEntity,
};

const CONFIG: Config = Config {
    endian: config::Endian::Little,
    address_width: config::AddressWidth::X64,
    double_support: true,
};

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct Unit0;

#[test]
fn unit0() {
    assert_eq!(Unit0::align(&CONFIG), 1);
    assert_eq!(Unit0::static_size(&CONFIG), 0);
}

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct Struct0 {
    pub x: u8,
    pub y: i32,
    pub z: (),
    pub w: [u8; 3],
}

#[test]
fn struct0() {
    assert_eq!(Struct0::align(&CONFIG), 4);
    assert_eq!(Struct0::static_size(&CONFIG), 12);
}

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
struct Tuple0(u8, i32, (), [u8; 3]);

#[test]
fn tuple0() {
    assert_eq!(Tuple0::align(&CONFIG), 4);
    assert_eq!(Tuple0::static_size(&CONFIG), 12);
}

#[derive(EntityId, Entity, SizedEntity, EntitySource)]
enum Enum0 {
    A { x: u8, y: f32 },
    B((), [u16; 3]),
    C,
}

#[test]
fn enum0() {
    assert_eq!(Enum0::align(&CONFIG), 8);
    assert_eq!(Enum0::static_size(&CONFIG), 16);
}
