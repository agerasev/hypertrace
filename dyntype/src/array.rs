use std::{
    io::{self, Read, Write},
    hash::{Hash, Hasher},
};
use crate::{Config, type_::*};

#[derive(Clone)]
pub struct ArrayType<T: Type> {
    item_type: T,
    item_count: usize,
}
impl<T: Type> ArrayType<T> {
    pub fn new(item_type: T, item_count: usize) -> Self {
        Self { item_type, item_count }
    }
}
impl<T: Type> TypeBase for ArrayType<T> {
    fn align(&self, config: &Config) -> usize {
        self.item_type.align(config)
    }
    fn size(&self, config: &Config) -> Option<usize> {
        Some(self.item_count * self.item_type.size(config).unwrap())
    }
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        type_id::<T>().hash(&mut hasher);
        self.item_type.id().hash(&mut hasher);
        self.item_count.hash(&mut hasher);
        hasher.finish()
    }
}
impl<T: Type> Type for ArrayType<T> {
    type Inst = ArrayInst<T::Inst>;

    fn load<R: Read + ?Sized>(&self, config: &Config, src: &mut R) -> io::Result<Self::Inst> {
        let mut inst = Self::Inst::new();
        for _ in 0..self.item_count {
            inst.push(self.item_type.load(config, src)?).map_err(|_| ()).unwrap();
        }
        Ok(inst)
    }
}
impl<T: Type> TypeSpec for ArrayType<T> {}

#[derive(Default)]
pub struct ArrayInst<T: Inst> {
    item_type: Option<T::Type>,
    items: Vec<T>,
}
impl<T: Inst> ArrayInst<T> {
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
        Ok(())
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn get(&self, pos: usize) -> Option<&T> {
        self.items.get(pos)
    }
    #[allow(clippy::result_unit_err)]
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
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
impl<T: Inst> InstBase for ArrayInst<T> {
    fn size(&self, config: &Config) -> usize {
        self.type_().size(config).unwrap()
    }
}
impl<T: Inst> Inst for ArrayInst<T> {
    type Type = ArrayType<T::Type>;

    fn type_(&self) -> Self::Type {
        ArrayType::new(self.item_type.as_ref().unwrap().clone(), self.len())
    }
    fn store<W: Write + ?Sized>(&self, config: &Config, dst: &mut W) -> io::Result<()> {
        for item in self.items.iter() {
            item.store(config, dst)?;
        }
        Ok(())
    }
}
impl<T: Inst> InstSpec for ArrayInst<T> {}

