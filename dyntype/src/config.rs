pub enum Endian {
    Little,
    Big,
}

pub enum AddressWidth {
    X32,
    X64,
}

pub struct Config {
    pub endian: Endian,
    pub address_width: AddressWidth,
    pub double_support: bool,
}
