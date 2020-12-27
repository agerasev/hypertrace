use std::{
    mem::size_of,
    io::{self, Read, Write},
    hash::Hash,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::{config::*, traits::*};

pub trait PrimitiveType: Type {}
pub trait PrimitiveValue: Value + Sized {}

macro_rules! impl_native {
    ($T:ident, $V:ident, $read:ident, $write:ident) => {
        impl BasicType for $T {
            fn id(&self, _: &Config) -> u64 {
                type_id::<Self>()
            }
            fn align(&self, _: &Config) -> usize {
                size_of::<Self>()
            }
            fn size(_: &Config) -> Option<usize> {
                Some(size_of::<Self>())
            }
            fn load<R: Read + ?Sized>(config: &Config, src: &mut R) -> io::Result<Self> {
                match config.endianness {
                    Endian::Big => src.$read::<BigEndian>(),
                    Endian::Little => src.$read::<LittleEndian>(),
                }
            }
            fn store<W: Write + ?Sized>(self, config: &Config, dst: &mut W) -> io::Result<()> {
                match config.endianness {
                    Endian::Big => dst.$write::<BigEndian>(self),
                    Endian::Little => dst.$write::<LittleEndian>(self),
                }
            }
        }
    };
}

macro_rules! impl_transfer_byte {
    ($T:ident, $read:ident, $write:ident) => {
        impl Transferable for $T {
            fn size(_: &Config) -> usize {
                size_of::<Self>()
            }
            fn load<R: Read + ?Sized>(_: &Config, src: &mut R) -> io::Result<Self> {
                src.$read()
            }
            fn store<W: Write + ?Sized>(self, _: &Config, dst: &mut W) -> io::Result<()> {
                dst.$write(self)
            }
        }
    };
}

macro_rules! impl_transfer_size {
    ($T:ident, $T32:ident, $T64:ident) => {
        impl Transferable for $T {
            fn size(config: &Config) -> usize {
                match config.address_width {
                    AddressWidth::X32 => <$T32 as Transferable>::size(config),
                    AddressWidth::X64 => <$T64 as Transferable>::size(config),
                }
            }
            fn load<R: Read + ?Sized>(config: &Config, src: &mut R) -> io::Result<Self> {
                match config.address_width {
                    AddressWidth::X32 => <$T32 as Transferable>::load(config, src).map(|x| x as $T),
                    AddressWidth::X64 => <$T64 as Transferable>::load(config, src).map(|x| x as $T),
                }
            }
            fn store<W: Write + ?Sized>(self, config: &Config, dst: &mut W) -> io::Result<()> {
                match config.address_width {
                    AddressWidth::X32 => <$T32 as Transferable>::store(self as $T32, config, dst),
                    AddressWidth::X64 => <$T64 as Transferable>::store(self as $T64, config, dst),
                }
            }
        }
    };
}

//impl_byte!(u8, read_u8, write_u8);
impl_native!(u16, read_u16, write_u16);
//impl_native!(u32, read_u32, write_u32);
//impl_native!(u64, read_u64, write_u64);
//
//impl_byte!(i8, read_i8, write_i8);
//impl_native!(i16, read_i16, write_i16);
//impl_native!(i32, read_i32, write_i32);
//impl_native!(i64, read_i64, write_i64);
//
//impl_size!(usize, u32, u64);
//impl_size!(isize, i32, i64);
//
//impl_native!(f32, read_f32, write_f32);

impl Transferable for f64 {
    fn size(config: &Config) -> usize {
        if config.double_support {
            size_of::<Self>()
        } else {
            <f32 as Transferable>::size(config)
        }
    }
    fn load<R: Read + ?Sized>(config: &Config, src: &mut R) -> io::Result<Self> {
        if config.double_support {
            match config.endianness {
                Endian::Big => src.read_f64::<BigEndian>(),
                Endian::Little => src.read_f64::<LittleEndian>(),
            }
        } else {
            <f32 as Transferable>::load(config, src).map(|x| x as f64)
        }
    }
    fn store<W: Write + ?Sized>(self, config: &Config, dst: &mut W) -> io::Result<()> {
        if config.double_support {
            match config.endianness {
                Endian::Big => dst.write_f64::<BigEndian>(self),
                Endian::Little => dst.write_f64::<LittleEndian>(self),
            }
        } else {
            <f32 as Transferable>::store(self as f32, config, dst)
        }
    }
}

macro_rules! impl_prim {
    ($Type:ident, $Value:ident) => {
        #[derive(Clone, Copy)]
        pub struct $Type;

        impl BasicType for $Type {
            fn align(&self, config: &Config) -> usize {
                <$Value as Transferable>::size(config)
            }
            fn size(&self, config: &Config) -> Option<usize> {
                Some(<$Value as Transferable>::size(config))
            }
            fn id(&self) -> u64 {
                type_id::<Self>()
            }
        }
        impl Type for $Type {
            type Value = $Value;

            fn load<R: Read + ?Sized>(&self, config: &Config, src: &mut R) -> io::Result<Self::Value> {
                <$Value as Transferable>::load(config, src)
            }
        }
        impl PrimitiveType for $Type {}

        impl BasicValue for $Value {
            fn size(&self, config: &Config) -> usize {
                <Self as Transferable>::size(config)
            }
        }
        impl Value for $Value {
            type Type = $Type;

            fn type_(&self) -> Self::Type {
                $Type
            }
            fn store<W: Write + ?Sized>(&self, config: &Config, dst: &mut W) -> io::Result<()> {
                <Self as Transferable>::store(*self, config, dst)
            }
        }
        impl PrimitiveValue for $Value {
            fn align(config: &Config) -> usize {
                <Self as Transferable>::align(config)
            }
            fn load<R: Read + ?Sized>(config: &Config, src: &mut R) -> io::Result<Self> {
                <Self as Transferable>::load(config, src)
            }
        }
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
