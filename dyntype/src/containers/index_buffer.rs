use crate::{traits::*, primitive::*, Config, util::*, containers::{TypedVec, util::*}};
use std::{
    hash::{Hash, Hasher},
    io::{self, Read, Write},
};

#[derive(Clone, Debug)]
pub struct IndexBufferType<T: Type>{
    item_type: T,
}

impl<T: Type> IndexBufferType<T>{
    pub fn new(item_type: T) -> Self {
        Self { item_type }
    }
}

impl<T: Type> Type for IndexBufferType<T> {
    type Value = IndexBuffer<T::Value>;

    fn align(&self, cfg: &Config) -> usize {
        lcm(self.item_type.align(cfg), UsizeType.align(cfg))
    }

    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        type_id::<Self>().hash(&mut hasher);
        self.item_type.id().hash(&mut hasher);
        hasher.finish()
    }

    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct IndexBuffer<V: Value> {
    inner: TypedVec<V>,
}

impl<V: Value> IndexBuffer<V> {
    pub fn new(item_type: V::Type) -> Self {
        Self {
            inner: TypedVec::new(item_type),
        }
    }

    pub fn from_items(item_type: V::Type, items: Vec<V>) -> Option<Self> {
        TypedVec::from_items(item_type, items).map(|v| Self { inner: v })
    }

    pub fn item_type(&self) -> &V::Type {
        &self.inner.item_type()
    }
    pub fn items(&self) -> &TypedVec<V> {
        &self.inner
    }
    pub fn items_mut(&mut self) -> &mut TypedVec<V> {
        &mut self.inner
    }

    pub fn into_items(self) -> Vec<V> {
        self.inner.into_items()
    }
}

impl<V: Value> Value for IndexBuffer<V> {
    type Type = IndexBufferType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        vector_size(&self.type_(), self.items.len(), cfg)
    }

    fn type_(&self) -> Self::Type {
        IndexBufferType::new(self.item_type.clone())
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        write_and_align(&self.items.len(), cfg, self.type_().align(cfg), dst)?;
        for item in self.items.iter() {
            item.store(cfg, dst)?;
        }
        Ok(())
    }
}
