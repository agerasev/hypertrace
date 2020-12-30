use crate::{traits::*, primitive::*, Config, util::*, containers::util::*};
use std::{
    hash::{Hash, Hasher},
    io::{self, Read, Write},
    marker::PhantomData,
};

#[derive(Clone, Debug)]
pub struct IndexVectorType<T: Type>{
    item_type: T,
}

impl<T: Type> IndexVectorType<T>{
    pub fn new(item_type: T) -> Self {
        Self { item_type }
    }
}

impl<T: Type> Type for IndexVectorType<T> {
    type Value = IndexVectorValue<T::Value>;

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

#[derive(Default, Debug)]
pub struct IndexVectorValue<V: Value> {
    item_type: V::Type,
    items: Vec<V>,
}

impl<V: Value> IndexVectorValue<V> {
    pub fn new(item_type: V::Type) -> Self {
        Self {
            item_type,
            items: Vec::new(),
        }
    }

    pub fn from_items(item_type: V::Type, items: Vec<V>) -> Option<Self> {
        if items.iter().all(|item| item.type_().id() == item_type.id()) {
            Some(Self { item_type, items })
        } else {
            None
        }
    }

    pub fn item_type(&self) -> &V::Type {
        &self.item_type
    }

    pub fn push(&mut self, item: V) -> Result<(), V> {
        if self.item_type.id() == item.type_().id() {
            self.items.push(item);
            Ok(())
        } else {
            Err(item)
        }
    }

    pub fn pop(&mut self) -> Option<V> {
        self.items.pop()
    }

    pub fn get(&self, pos: usize) -> Option<&V> {
        self.items.get(pos)
    }

    #[allow(clippy::result_unit_err)]
    pub fn update<F: FnOnce(&mut V)>(&mut self, pos: usize, f: F) -> Result<(), ()> {
        let item = self.items.get_mut(pos).ok_or(())?;
        f(item);
        assert_eq!(item.type_().id(), self.item_type.id());
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn into_items(self) -> Vec<V> {
        self.items
    }
}

fn vector_size<T: Type>(type_: &IndexVectorType<T>, len: usize, cfg: &Config) -> usize
where
    T::Value: Value
{
    upper_multiple(UsizeType.size(cfg), type_.align(cfg)) + len * type_.item_type.size(cfg)
}

impl<V: Value> Value for IndexVectorValue<V> {
    type Type = IndexVectorType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        vector_size(&self.type_(), self.items.len(), cfg)
    }

    fn type_(&self) -> Self::Type {
        IndexVectorType::new(self.item_type.clone())
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        write_and_align(&self.items.len(), cfg, self.type_().align(cfg), dst)?;
        for item in self.items.iter() {
            item.store(cfg, dst)?;
        }
        Ok(())
    }
}
