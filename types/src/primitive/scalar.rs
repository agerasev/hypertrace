use crate::{
    config::{AddressWidth, Config, Endian},
    io::{CntRead, CntWrite},
    Entity, SizedEntity, SourceInfo,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{fmt::Debug, io, mem::size_of};
use num_traits::Num;

pub trait PrimScal: SizedEntity + Num + Copy + Default + Debug + 'static {}

macro_rules! impl_entity_native {
    ($T:ident, $K:expr, $read:ident, $write:ident) => {
        impl PrimScal for $T {}
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::type_size(cfg)
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
            fn entity_source(_: &Config) -> SourceInfo {
                SourceInfo::with_root($K, $K, "types.hh")
            }
        }
        impl SizedEntity for $T {
            fn type_size(_cfg: &Config) -> usize {
                size_of::<Self>()
            }
        }
    };
}

macro_rules! impl_entity_byte {
    ($T:ident, $K:expr, $read:ident, $write:ident) => {
        impl PrimScal for $T {}
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn load<R: CntRead>(_cfg: &Config, src: &mut R) -> io::Result<Self> {
                src.$read()
            }
            fn store<W: CntWrite>(&self, _cfg: &Config, dst: &mut W) -> io::Result<()> {
                dst.$write(*self)
            }
            fn entity_source(_: &Config) -> SourceInfo {
                SourceInfo::with_root($K, $K, "types.hh")
            }
        }
        impl SizedEntity for $T {
            fn type_size(_cfg: &Config) -> usize {
                1
            }
        }
    };
}

macro_rules! impl_entity_size {
    ($T:ident, $T32:ident, $T64:ident) => {
        impl PrimScal for $T {}
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::type_size(cfg)
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
            fn entity_source(cfg: &Config) -> SourceInfo {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::entity_source(cfg),
                    AddressWidth::X64 => $T64::entity_source(cfg),
                }
            }
        }
        impl SizedEntity for $T {
            fn type_size(cfg: &Config) -> usize {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::type_size(cfg),
                    AddressWidth::X64 => $T64::type_size(cfg),
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

impl_entity_size!(usize, u32, u64);
impl_entity_size!(isize, i32, i64);

impl_entity_native!(f32, "float", read_f32, write_f32);

impl PrimScal for f64 {}
impl Entity for f64 {
    fn align(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
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
    fn entity_source(cfg: &Config) -> SourceInfo {
        if cfg.double_support {
            SourceInfo::with_root("double", "double", "types.hh")
        } else {
            f32::entity_source(cfg)
        }
    }
}
impl SizedEntity for f64 {
    fn type_size(cfg: &Config) -> usize {
        if cfg.double_support {
            size_of::<Self>()
        } else {
            f32::type_size(cfg)
        }
    }
}