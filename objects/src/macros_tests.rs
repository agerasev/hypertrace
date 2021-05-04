use type_macros::{Named, SizedEntity, Sourced};
use types::{Entity, SizedEntity, config::{self, Config}};

const CONFIG: Config = Config {
    endian: config::Endian::Little,
    address_width: config::AddressWidth::X64,
    double_support: true,
};

#[derive(Named, SizedEntity, Sourced)]
struct Unit0;

#[test]
fn unit0() {
    assert_eq!(Unit0::align(&CONFIG), 1);
    assert_eq!(Unit0::type_size(&CONFIG), 0);
}

#[derive(Named, SizedEntity, Sourced)]
struct Struct0 {
    pub x: u8,
    pub y: i32,
    pub z: (),
    pub w: [u8; 3],
}

#[test]
fn struct0() {
    assert_eq!(Struct0::align(&CONFIG), 4);
    assert_eq!(Struct0::type_size(&CONFIG), 12);
}

#[derive(Named, SizedEntity, Sourced)]
struct Tuple0(u8, i32, (), [u8; 3]);

#[test]
fn tuple0() {
    assert_eq!(Tuple0::align(&CONFIG), 4);
    assert_eq!(Tuple0::type_size(&CONFIG), 12);
}

#[derive(Named, SizedEntity, Sourced)]
enum Enum0 {
    A { x: u8, y: f32 },
    B((), [u16; 3]),
    C,
}

#[test]
fn enum0() {
    assert_eq!(Enum0::align(&CONFIG), 8);
    assert_eq!(Enum0::type_size(&CONFIG), 16);
}
