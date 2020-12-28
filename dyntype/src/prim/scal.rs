use crate::{config::*, traits::*};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{self, Read, Write},
    mem::size_of,
};

trait Xfer: Sized {
    fn len(cfg: &Config) -> usize;
    fn rx<R: Read + ?Sized>(cfg: &Config, src: &mut R) -> io::Result<Self>;
    fn tx<W: Write + ?Sized>(self, cfg: &Config, dst: &mut W) -> io::Result<()>;
}

macro_rules! impl_xfer_native {
    ($T:ident, $read:ident, $write:ident) => {
        impl Xfer for $T {
            fn len(_: &Config) -> usize {
                size_of::<Self>()
            }
            fn rx<R: Read + ?Sized>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                match cfg.endian {
                    Endian::Big => src.$read::<BigEndian>(),
                    Endian::Little => src.$read::<LittleEndian>(),
                }
            }
            fn tx<W: Write + ?Sized>(self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                match cfg.endian {
                    Endian::Big => dst.$write::<BigEndian>(self),
                    Endian::Little => dst.$write::<LittleEndian>(self),
                }
            }
        }
    };
}

macro_rules! impl_xfer_byte {
    ($T:ident, $read:ident, $write:ident) => {
        impl Xfer for $T {
            fn len(_: &Config) -> usize {
                1
            }
            fn rx<R: Read + ?Sized>(_: &Config, src: &mut R) -> io::Result<Self> {
                src.$read()
            }
            fn tx<W: Write + ?Sized>(self, _: &Config, dst: &mut W) -> io::Result<()> {
                dst.$write(self)
            }
        }
    };
}

macro_rules! impl_xfer_size {
    ($T:ident, $T32:ident, $T64:ident) => {
        impl Xfer for $T {
            fn len(cfg: &Config) -> usize {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::len(cfg),
                    AddressWidth::X64 => $T64::len(cfg),
                }
            }
            fn rx<R: Read + ?Sized>(cfg: &Config, src: &mut R) -> io::Result<Self> {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::rx(cfg, src).map(|x| x as $T),
                    AddressWidth::X64 => $T64::rx(cfg, src).map(|x| x as $T),
                }
            }
            fn tx<W: Write + ?Sized>(self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                match cfg.address_width {
                    AddressWidth::X32 => $T32::tx(self as $T32, cfg, dst),
                    AddressWidth::X64 => $T64::tx(self as $T64, cfg, dst),
                }
            }
        }
    };
}

impl_xfer_byte!(u8, read_u8, write_u8);
impl_xfer_native!(u16, read_u16, write_u16);
impl_xfer_native!(u32, read_u32, write_u32);
impl_xfer_native!(u64, read_u64, write_u64);

impl_xfer_byte!(i8, read_i8, write_i8);
impl_xfer_native!(i16, read_i16, write_i16);
impl_xfer_native!(i32, read_i32, write_i32);
impl_xfer_native!(i64, read_i64, write_i64);

impl_xfer_size!(usize, u32, u64);
impl_xfer_size!(isize, i32, i64);

impl_xfer_native!(f32, read_f32, write_f32);

impl Xfer for f64 {
    fn len(cfg: &Config) -> usize {
        if cfg.double_support {
            size_of::<Self>()
        } else {
            f32::len(cfg)
        }
    }
    fn rx<R: Read + ?Sized>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        if cfg.double_support {
            match cfg.endian {
                Endian::Big => src.read_f64::<BigEndian>(),
                Endian::Little => src.read_f64::<LittleEndian>(),
            }
        } else {
            f32::rx(cfg, src).map(|x| x as f64)
        }
    }
    fn tx<W: Write + ?Sized>(self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        if cfg.double_support {
            match cfg.endian {
                Endian::Big => dst.write_f64::<BigEndian>(self),
                Endian::Little => dst.write_f64::<LittleEndian>(self),
            }
        } else {
            f32::tx(self as f32, cfg, dst)
        }
    }
}

pub trait PrimType: SizedType
where
    Self::Value: PrimValue<Type = Self>,
{
}
pub trait PrimValue: SizedValue
where
    Self::Type: PrimType<Value = Self>,
{
}

macro_rules! impl_prim {
    ($T:ident, $V:ident) => {
        #[derive(Clone, Copy)]
        pub struct $T;

        impl Type for $T {
            type Value = $V;

            fn align(&self, cfg: &Config) -> usize {
                $V::len(cfg)
            }

            fn id(&self) -> u64 {
                type_id::<Self>()
            }

            fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
                $V::rx(cfg, src)
            }
        }

        impl SizedType for $T {
            fn size(&self, cfg: &Config) -> usize {
                $V::len(cfg)
            }
        }

        impl PrimType for $T {}

        impl Value for $V {
            type Type = $T;

            fn size(&self, cfg: &Config) -> usize {
                Self::len(cfg)
            }

            fn type_(&self) -> Self::Type {
                $T
            }
            fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
                Self::tx(*self, cfg, dst)
            }
        }

        impl SizedValue for $V {}

        impl PrimValue for $V {}
    };
}

impl_prim!(U8Type, u8);
impl_prim!(U16Type, u16);
impl_prim!(U32Type, u32);
impl_prim!(U64Type, u64);

impl_prim!(I8Type, i8);
impl_prim!(I16Type, i16);
impl_prim!(I32Type, i32);
impl_prim!(I64Type, i64);

impl_prim!(UsizeType, usize);
impl_prim!(IsizeType, isize);

impl_prim!(F32Type, f32);
impl_prim!(F64Type, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        assert_ne!(I32Type.id(), U32Type.id());
    }
}
