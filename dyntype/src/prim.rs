use std::{
    mem::size_of,
    io::{self, Read, Write},
    hash::Hash,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::{config::*, type_::*};


pub trait PrimType: Type {}
pub trait PrimInst: Inst {}

trait Transferable: Sized {
    fn align(config: &Config) -> usize {
        Self::size(config)
    }
    fn size(config: &Config) -> usize;
    fn load<R: Read + ?Sized>(config: &Config, src: &mut R) -> io::Result<Self>;
    fn store<W: Write + ?Sized>(self, config: &Config, dst: &mut W) -> io::Result<()>;
}

macro_rules! impl_transfer_native {
    ($T:ident, $read:ident, $write:ident) => {
        impl Transferable for $T {
            fn size(_: &Config) -> usize {
                size_of::<Self>()
            }
            fn load<R: Read + ?Sized>(config: &Config, src: &mut R) -> io::Result<Self> {
                match config.endianness {
                    Endianness::Big => src.$read::<BigEndian>(),
                    Endianness::Little => src.$read::<LittleEndian>(),
                }
            }
            fn store<W: Write + ?Sized>(self, config: &Config, dst: &mut W) -> io::Result<()> {
                match config.endianness {
                    Endianness::Big => dst.$write::<BigEndian>(self),
                    Endianness::Little => dst.$write::<LittleEndian>(self),
                }
            }
        }
    };
}

macro_rules! impl_transfer_byte {
    ($T:ident, $read:ident, $write:ident) => {
        impl Transferable for $T {
            fn size(_: &Config) -> usize {
                1
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

impl_transfer_byte!(u8, read_u8, write_u8);
impl_transfer_native!(u16, read_u16, write_u16);
impl_transfer_native!(u32, read_u32, write_u32);
impl_transfer_native!(u64, read_u64, write_u64);

impl_transfer_byte!(i8, read_i8, write_i8);
impl_transfer_native!(i16, read_i16, write_i16);
impl_transfer_native!(i32, read_i32, write_i32);
impl_transfer_native!(i64, read_i64, write_i64);

impl_transfer_native!(f32, read_f32, write_f32);

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
                Endianness::Big => src.read_f64::<BigEndian>(),
                Endianness::Little => src.read_f64::<LittleEndian>(),
            }
        } else {
            <f32 as Transferable>::load(config, src).map(|x| x as f64)
        }
    }
    fn store<W: Write + ?Sized>(self, config: &Config, dst: &mut W) -> io::Result<()> {
        if config.double_support {
            match config.endianness {
                Endianness::Big => dst.write_f64::<BigEndian>(self),
                Endianness::Little => dst.write_f64::<LittleEndian>(self),
            }
        } else {
            <f32 as Transferable>::store(self as f32, config, dst)
        }
    }
}

macro_rules! impl_prim {
    ($Type:ident, $Inst:ident) => {
        #[derive(Clone, Copy, Hash)]
        pub struct $Type;

        impl TypeBase for $Type {
            fn align(&self, config: &Config) -> usize {
                <$Inst as Transferable>::size(config)
            }
            fn size(&self, config: &Config) -> Option<usize> {
                Some(<$Inst as Transferable>::size(config))
            }
        }
        impl Type for $Type {
            type Inst = $Inst;

            fn load<R: Read + ?Sized>(&self, config: &Config, src: &mut R) -> io::Result<Self::Inst> {
                <$Inst as Transferable>::load(config, src)
            }
        }
        impl TypeSpec for $Type {}
        impl PrimType for $Type {}

        impl InstBase for $Inst {
            fn size(&self, config: &Config) -> usize {
                <Self as Transferable>::size(config)
            }
        }
        impl Inst for $Inst {
            type Type = $Type;

            fn type_(&self) -> Self::Type {
                $Type
            }
            fn store<W: Write + ?Sized>(&self, config: &Config, dst: &mut W) -> io::Result<()> {
                <Self as Transferable>::store(*self, config, dst)
            }
        }
        impl InstSpec for $Inst {}
        impl PrimInst for $Inst {}
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
