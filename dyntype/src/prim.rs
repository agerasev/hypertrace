use std::{
    io::{self, Read, Write},
    hash::Hash,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::type_::*;

pub trait PrimType: Type {}
pub trait PrimInst: Inst {}

#[derive(Copy, Clone, Hash)]
pub struct I32Type;

impl TypeBase for I32Type {
    fn align(&self) -> usize {
        4
    }
    fn size(&self) -> Option<usize> {
        Some(4)
    }
}
impl Type for I32Type {
    type Inst = i32;

    fn load<R: Read + ?Sized>(&self, src: &mut R) -> io::Result<Self::Inst> {
        src.read_i32::<LittleEndian>()
    }
}
impl TypeSpec for I32Type {}
impl PrimType for I32Type {}

impl InstBase for i32 {
    fn size(&self) -> usize {
        self.type_().size().unwrap()
    }
}
impl Inst for i32 {
    type Type = I32Type;

    fn type_(&self) -> Self::Type {
        I32Type
    }
    fn store<W: Write + ?Sized>(&self, dst: &mut W) -> io::Result<()> {
        dst.write_i32::<LittleEndian>(*self)
    }
}
impl InstSpec for i32 {}
impl PrimInst for i32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        assert_ne!(I32Type.id(), 0);
    }
}
