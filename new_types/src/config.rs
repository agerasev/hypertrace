use byteorder::{BigEndian, LittleEndian, NativeEndian};

pub enum Endian {
    Little,
    Big,
}
trait ToEndian {
    const ENDIAN: Endian;
}
impl ToEndian for LittleEndian {
    const ENDIAN: Endian = Endian::Little;
}
impl ToEndian for BigEndian {
    const ENDIAN: Endian = Endian::Big;
}

pub enum AddressWidth {
    X32,
    X64,
}

#[cfg(target_pointer_width = "32")]
const HOST_ADDRESS_WIDTH: AddressWidth = AddressWidth::X32;
#[cfg(target_pointer_width = "64")]
const HOST_ADDRESS_WIDTH: AddressWidth = AddressWidth::X64;

pub struct Config {
    pub endian: Endian,
    pub address_width: AddressWidth,
    pub double_support: bool,
}

pub const HOST_CONFIG: Config = Config {
    endian: NativeEndian::ENDIAN,
    address_width: HOST_ADDRESS_WIDTH,
    double_support: true,
};
