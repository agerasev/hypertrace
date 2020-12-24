pub enum Endianness {
    Little,
    Big,
}

pub enum AddressWidth {
    X32,
    X64,
}

pub struct Config {
    pub endianness: Endianness,
    pub address_width: AddressWidth,
    pub double_support: bool,
}
