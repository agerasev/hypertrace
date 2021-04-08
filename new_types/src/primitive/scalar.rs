use crate::{
    config::{AddressWidth, Config, Endian},
    io::{CountingRead, CountingWrite},
    Entity, SizedEntity, SourceTree,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{fmt::Debug, io, mem::size_of};

pub trait PrimScal: SizedEntity + Copy + Default + Debug + 'static {}

macro_rules! impl_entity_native {
    ($T:ident, $read:ident, $write:ident) => {
        impl PrimScal for $T {}
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn load<R: CountingRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                match cfg.endian {
                    Endian::Big => src.$read::<BigEndian>(),
                    Endian::Little => src.$read::<LittleEndian>(),
                }
            }
            fn store<W: CountingWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                match cfg.endian {
                    Endian::Big => dst.$write::<BigEndian>(*self),
                    Endian::Little => dst.$write::<LittleEndian>(*self),
                }
            }
            fn source(_: &Config) -> Option<SourceTree> {
                None
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
    ($T:ident, $read:ident, $write:ident) => {
        impl PrimScal for $T {}
        impl Entity for $T {
            fn align(cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn size(&self, cfg: &Config) -> usize {
                Self::type_size(cfg)
            }
            fn load<R: CountingRead>(_cfg: &Config, src: &mut R) -> io::Result<Self> {
                src.$read()
            }
            fn store<W: CountingWrite>(&self, _cfg: &Config, dst: &mut W) -> io::Result<()> {
                dst.$write(*self)
            }
            fn source(_: &Config) -> Option<SourceTree> {
                None
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
            fn load<R: CountingRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::load(cfg, src).map(|x| x as $T),
                    AddressWidth::X64 => $T64::load(cfg, src).map(|x| x as $T),
                }
            }
            fn store<W: CountingWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                match cfg.address_width {
                    AddressWidth::X32 => (*self as $T32).store(cfg, dst),
                    AddressWidth::X64 => (*self as $T64).store(cfg, dst),
                }
            }
            fn source(_: &Config) -> Option<SourceTree> {
                None
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

impl_entity_byte!(u8, read_u8, write_u8);
impl_entity_native!(u16, read_u16, write_u16);
impl_entity_native!(u32, read_u32, write_u32);
impl_entity_native!(u64, read_u64, write_u64);

impl_entity_byte!(i8, read_i8, write_i8);
impl_entity_native!(i16, read_i16, write_i16);
impl_entity_native!(i32, read_i32, write_i32);
impl_entity_native!(i64, read_i64, write_i64);

impl_entity_size!(usize, u32, u64);
impl_entity_size!(isize, i32, i64);

impl_entity_native!(f32, read_f32, write_f32);

impl PrimScal for f64 {}
impl Entity for f64 {
    fn align(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }
    fn load<R: CountingRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        if cfg.double_support {
            match cfg.endian {
                Endian::Big => src.read_f64::<BigEndian>(),
                Endian::Little => src.read_f64::<LittleEndian>(),
            }
        } else {
            f32::load(cfg, src).map(|x| x as f64)
        }
    }
    fn store<W: CountingWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        if cfg.double_support {
            match cfg.endian {
                Endian::Big => dst.write_f64::<BigEndian>(*self),
                Endian::Little => dst.write_f64::<LittleEndian>(*self),
            }
        } else {
            (*self as f32).store(cfg, dst)
        }
    }
    fn source(_: &Config) -> Option<SourceTree> {
        None
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
