use crate::{
    config::{AddressWidth, Config, Endian},
    io::{CntRead, CntWrite},
    source::SourceTree,
    Entity, EntityId, SizedEntity, EntitySource,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::Num;
use std::{fmt::Debug, io, mem::size_of};

pub trait PrimScal: SizedEntity + Num + Copy + Default + Debug + 'static {}

macro_rules! impl_entity_native {
    ($T:ident, $K:expr, $read:ident, $write:ident) => {
        impl PrimScal for $T {}
        impl EntityId for $T {
            fn name() -> (String, String) {
                (String::from($K), String::from($K))
            }
        }
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn min_size(cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn is_dyn_sized() -> bool {
                false
            }
            fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                match cfg.endian {
                    Endian::Big => src.$read::<BigEndian>(),
                    Endian::Little => src.$read::<LittleEndian>(),
                }
            }
            fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                match cfg.endian {
                    Endian::Big => dst.$write::<BigEndian>(*self),
                    Endian::Little => dst.$write::<LittleEndian>(*self),
                }
            }
        }
        impl SizedEntity for $T {
            fn static_size(_cfg: &Config) -> usize {
                size_of::<Self>()
            }
        }
        impl EntitySource for $T {
            fn source(_: &Config) -> SourceTree {
                SourceTree::new("types.hh")
            }
        }
    };
}

macro_rules! impl_entity_byte {
    ($T:ident, $K:expr, $read:ident, $write:ident) => {
        impl PrimScal for $T {}
        impl EntityId for $T {
            fn name() -> (String, String) {
                (String::from($K), String::from($K))
            }
        }
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn min_size(cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn is_dyn_sized() -> bool {
                false
            }
            fn load<R: CntRead>(_cfg: &Config, src: &mut R) -> io::Result<Self> {
                src.$read()
            }
            fn store<W: CntWrite>(&self, _cfg: &Config, dst: &mut W) -> io::Result<()> {
                dst.$write(*self)
            }
        }
        impl SizedEntity for $T {
            fn static_size(_cfg: &Config) -> usize {
                1
            }
        }
        impl EntitySource for $T {
            fn source(_: &Config) -> SourceTree {
                SourceTree::new("types.hh")
            }
        }
    };
}

macro_rules! impl_entity_size {
    ($T:ident, $K:expr, $T32:ident, $T64:ident) => {
        impl PrimScal for $T {}
        impl EntityId for $T {
            fn name() -> (String, String) {
                (String::from($K), String::from($K))
            }
        }
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn min_size(cfg: &Config) -> usize {
                Self::static_size(cfg)
            }
            fn is_dyn_sized() -> bool {
                false
            }
            fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::load(cfg, src).map(|x| x as $T),
                    AddressWidth::X64 => $T64::load(cfg, src).map(|x| x as $T),
                }
            }
            fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                match cfg.address_width {
                    AddressWidth::X32 => (*self as $T32).store(cfg, dst),
                    AddressWidth::X64 => (*self as $T64).store(cfg, dst),
                }
            }
        }
        impl SizedEntity for $T {
            fn static_size(cfg: &Config) -> usize {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::static_size(cfg),
                    AddressWidth::X64 => $T64::static_size(cfg),
                }
            }
        }
        impl EntitySource for $T {
            fn source(cfg: &Config) -> SourceTree {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::source(cfg),
                    AddressWidth::X64 => $T64::source(cfg),
                }
            }
        }
    };
}

impl_entity_byte!(u8, "uchar", read_u8, write_u8);
impl_entity_native!(u16, "ushort", read_u16, write_u16);
impl_entity_native!(u32, "uint", read_u32, write_u32);
impl_entity_native!(u64, "ulong", read_u64, write_u64);

impl_entity_byte!(i8, "char", read_i8, write_i8);
impl_entity_native!(i16, "short", read_i16, write_i16);
impl_entity_native!(i32, "int", read_i32, write_i32);
impl_entity_native!(i64, "long", read_i64, write_i64);

impl_entity_size!(usize, "usize", u32, u64);
impl_entity_size!(isize, "isize", i32, i64);

impl_entity_native!(f32, "float", read_f32, write_f32);

impl PrimScal for f64 {}
impl EntityId for f64 {
    fn name() -> (String, String) {
        (String::from("real"), String::from("real"))
    }
}
impl Entity for f64 {
    fn align(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn min_size(cfg: &Config) -> usize {
        Self::static_size(cfg)
    }
    fn is_dyn_sized() -> bool {
        false
    }
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        if cfg.double_support {
            match cfg.endian {
                Endian::Big => src.read_f64::<BigEndian>(),
                Endian::Little => src.read_f64::<LittleEndian>(),
            }
        } else {
            f32::load(cfg, src).map(|x| x as f64)
        }
    }
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        if cfg.double_support {
            match cfg.endian {
                Endian::Big => dst.write_f64::<BigEndian>(*self),
                Endian::Little => dst.write_f64::<LittleEndian>(*self),
            }
        } else {
            (*self as f32).store(cfg, dst)
        }
    }
}
impl SizedEntity for f64 {
    fn static_size(cfg: &Config) -> usize {
        if cfg.double_support {
            size_of::<Self>()
        } else {
            f32::static_size(cfg)
        }
    }
}
impl EntitySource for f64 {
    fn source(cfg: &Config) -> SourceTree {
        if cfg.double_support {
            SourceTree::new("types.hh")
        } else {
            f32::source(cfg)
        }
    }
}
