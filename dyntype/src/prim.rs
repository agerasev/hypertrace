use std::{
    io::{self, Read, Write},
    hash::Hash,
};
use crate::type_::*;


#[derive(Clone, Hash)]
pub struct PrimType<T: Type> {
    item_type: T,
    item_count: usize,
}
impl<T: Type> PrimType<T> {
    pub fn new(item_type: T, item_count: usize) -> Self {
        Self { item_type, item_count }
    }
}
impl<T: Type> TypeBase for PrimType<T> {
    fn align(&self) -> usize {
        self.item_type.align()
    }
    fn size(&self) -> Option<usize> {
        Some(self.item_count * self.item_type.size().unwrap())
    }
}
impl<T: Type> Type for PrimType<T> {
    type Inst = PrimInst<T::Inst>;

    fn load<R: Read + ?Sized>(&self, src: &mut R) -> io::Result<Self::Inst> {
        let mut inst = Self::Inst::new();
        for _ in 0..self.item_count {
            inst.push(self.item_type.load(src)?).map_err(|_| ()).unwrap();
        }
        Ok(inst)
    }
}
impl<T: Type> TypeSpec for PrimType<T> {}


pub struct PrimInst<T: Inst> {
    item_type: Option<T::Type>,
    items: Vec<T>,
}
impl<T: Inst> PrimInst<T> {
    pub fn new() -> Self {
        Self {
            item_type: None,
            items: Vec::new()
        }
    }
    pub fn new_with_type(type_: T::Type) -> Self {
        Self {
            item_type: Some(type_),
            items: Vec::new()
        }
    }

    pub fn item_type(&self) -> Option<&T::Type> {
        self.item_type.as_ref()
    }
    
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.item_type.is_none() {
            assert!(self.items.is_empty());
            self.item_type = Some(item.type_());
        } else if self.item_type.as_ref().unwrap().id() != item.type_().id() {
            return Err(item);
        }
        self.items.push(item);
        return Ok(())
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn get(&self, pos: usize) -> Option<&T> {
        self.items.get(pos)
    }
    pub fn update<F: FnOnce(&mut T)>(&mut self, pos: usize, f: F) -> Result<(), ()> {
        let item = self.items.get_mut(pos).ok_or(())?;
        f(item);
        assert_eq!(item.type_().id(), self.item_type.as_ref().unwrap().id());
        Ok(())
    }
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.items.iter()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
impl<T: Inst> InstBase for PrimInst<T> {
    fn size(&self) -> usize {
        self.type_().size().unwrap()
    }
}
impl<T: Inst> Inst for PrimInst<T> {
    type Type = PrimType<T::Type>;

    fn type_(&self) -> Self::Type {
        PrimType::new(self.item_type.as_ref().unwrap().clone(), self.len())
    }
    fn store<W: Write + ?Sized>(&self, dst: &mut W) -> io::Result<()> {
        for item in self.items.iter() {
            item.store(dst)?;
        }
        Ok(())
    }
}
impl<T: Inst> InstSpec for PrimInst<T> {}

