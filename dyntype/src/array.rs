use crate::{traits::*, Config};
use std::{
    hash::{Hash, Hasher},
    io::{self, Read, Write},
};

#[derive(Clone, Debug)]
pub struct ArrayType<T: SizedType>
where
    T::Value: SizedValue,
{
    item_type: T,
    item_count: usize,
}

impl<T: SizedType> ArrayType<T>
where
    T::Value: SizedValue,
{
    pub fn new(item_type: T, item_count: usize) -> Self {
        Self {
            item_type,
            item_count,
        }
    }
}

impl<T: SizedType> BasicType for ArrayType<T>
where
    T::Value: SizedValue,
{
    fn align(&self, cfg: &Config) -> usize {
        self.item_type.align(cfg)
    }
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        type_id::<Self>().hash(&mut hasher);
        self.item_type.id().hash(&mut hasher);
        self.item_count.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: SizedType> BasicSizedType for ArrayType<T>
where
    T::Value: SizedValue,
{
    fn size(&self, cfg: &Config) -> usize {
        self.item_count * self.item_type.size(cfg)
    }
}

impl<T: SizedType> Type for ArrayType<T>
where
    T::Value: SizedValue,
{
    type Value = ArrayValue<T::Value>;

    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let mut inst = Self::Value::new();
        for _ in 0..self.item_count {
            inst.push(self.item_type.load(cfg, src)?)
                .map_err(|_| ())
                .unwrap();
        }
        Ok(inst)
    }
}

impl<T: SizedType> SizedType for ArrayType<T> where T::Value: SizedValue {}

#[derive(Default, Debug)]
pub struct ArrayValue<V: SizedValue>
where
    V::Type: SizedType,
{
    item_type: Option<V::Type>,
    items: Vec<V>,
}

impl<V: SizedValue> ArrayValue<V>
where
    V::Type: SizedType,
{
    pub fn new() -> Self {
        Self {
            item_type: None,
            items: Vec::new(),
        }
    }

    pub fn new_with_type(type_: V::Type) -> Self {
        Self {
            item_type: Some(type_),
            items: Vec::new(),
        }
    }

    pub fn item_type(&self) -> Option<&V::Type> {
        self.item_type.as_ref()
    }

    pub fn push(&mut self, item: V) -> Result<(), V> {
        if self.item_type.is_none() {
            assert!(self.items.is_empty());
            self.item_type = Some(item.type_());
        } else if self.item_type.as_ref().unwrap().id() != item.type_().id() {
            return Err(item);
        }
        self.items.push(item);
        Ok(())
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
        assert_eq!(item.type_().id(), self.item_type.as_ref().unwrap().id());
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
}

impl<V: SizedValue> BasicValue for ArrayValue<V>
where
    V::Type: SizedType,
{
    fn size(&self, cfg: &Config) -> usize {
        self.type_().size(cfg)
    }
}

impl<V: SizedValue> BasicSizedValue for ArrayValue<V>
where
    V::Type: SizedType,
{
}

impl<V: SizedValue> Value for ArrayValue<V>
where
    V::Type: SizedType,
{
    type Type = ArrayType<V::Type>;

    fn type_(&self) -> Self::Type {
        ArrayType::new(self.item_type.as_ref().unwrap().clone(), self.len())
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for item in self.items.iter() {
            item.store(cfg, dst)?;
        }
        Ok(())
    }
}

impl<V: SizedValue> SizedValue for ArrayValue<V> where V::Type: SizedType {}
