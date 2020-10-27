use std::{
    io::{self, Read, Write},
    hash::Hash,
};
use crate::type_::*;

#[derive(Clone, Hash)]
pub struct ArrayType<T: Type> {
    item_type: T,
    item_count: usize,
}
impl<T: Type> TypeBase for ArrayType<T> {
    fn align(&self) -> usize {
        self.item_type.align()
    }
    fn size(&self) -> Option<usize> {
        Some(self.item_count * self.item_type.size().unwrap())
    }
}
impl<T: Type> Type for ArrayType<T> {
    type Inst = ArrayInst<T::Inst>;

    fn load<R: Read + ?Sized>(&self, src: &mut R) -> io::Result<Self::Inst> {
        let mut inst = Self::Inst::new();
        for i in 0..self.item_count {
            inst.items.push(self.item_type.load(src)?);
        }
        Ok(inst)
    }
}
impl Clone for Box<dyn TypeDyn> {
    fn clone(&self) -> Self {
        self.as_ref().clone_dyn()
    }
}

pub struct ArrayInst<T: Inst> {
    items: Vec<T>,
}
impl<T: Inst> ArrayInst<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn push(item: T) -> Result<(), T> {
        if self.items.len() > 0 && self.items[0].type_() == item.type() {
            
        }
    }
}
impl<T: Inst> InstBase for ArrayInst<T> {
    fn size(&self) -> usize {
        self.as_ref().size()
    }
}
impl<T: Inst> Inst for ArrayInst<T> {
    type Type = ArrayType<T::Type>;

    fn type_(&self) -> Self::Type {
        self.as_ref().type_dyn()
    }
    fn store<W: Write + ?Sized>(&self, dst: &mut W) -> io::Result<()> {
        let mut wb = Box::new(dst) as Box<dyn Write>;
        self.as_ref().store_dyn(&mut wb)
    }
}
