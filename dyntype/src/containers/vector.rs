use crate::{traits::*, primitive::*, Config, util::*, containers::util::*};
use std::{
    hash::{Hash, Hasher},
    io::{self, Read, Write},
    marker::PhantomData,
};

#[derive(Clone, Debug)]
pub struct VectorType<T: SizedType>
where
    T::Value: SizedValue,
{
    item_type: T,
}

impl<T: SizedType> VectorType<T>
where
    T::Value: SizedValue,
{
    pub fn new(item_type: T) -> Self {
        Self { item_type }
    }
}

impl<T: SizedType> Type for VectorType<T> 
where
    T::Value: SizedValue,
{
    type Value = VectorValue<T::Value>;

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
        let mut value = Self::Value::new(self.item_type.clone());
        let len = read_and_align(&UsizeType, cfg, self.align(cfg), src)?;
        for _ in 0..len {
            value
                .push(self.item_type.load(cfg, src)?)
                .map_err(|_| ())
                .unwrap();
        }
        Ok(value)
    }
}

#[derive(Default, Debug)]
pub struct VectorValue<V: SizedValue> 
where
    V::Type: SizedType,
{
    item_type: V::Type,
    items: Vec<V>,
}

impl<V: SizedValue> VectorValue<V> 
where
    V::Type: SizedType,
{
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

fn vector_size<T: SizedType>(type_: &VectorType<T>, len: usize, cfg: &Config) -> usize
where
    T::Value: SizedValue
{
    upper_multiple(UsizeType.size(cfg), type_.align(cfg)) + len * type_.item_type.size(cfg)
}

impl<V: SizedValue> Value for VectorValue<V> 
where
    V::Type: SizedType,
{
    type Type = VectorType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        vector_size(&self.type_(), self.items.len(), cfg)
    }

    fn type_(&self) -> Self::Type {
        VectorType::new(self.item_type.clone())
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        write_and_align(&self.items.len(), cfg, self.type_().align(cfg), dst)?;
        for item in self.items.iter() {
            item.store(cfg, dst)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct StaticVectorType<T: SizedType + UnitType>
where
    T::Value: SizedValue + Default + Clone,
{
    phantom: PhantomData<T>,
}

impl<T: SizedType + UnitType> StaticVectorType<T>
where
    T::Value: SizedValue + Default + Clone,
{
    pub fn into_dynamic(self) -> VectorType<T> {
        VectorType::new(T::default())
    }
}

impl<T: SizedType + UnitType> Type for StaticVectorType<T>
where
    T::Value: SizedValue + Default + Clone,
{
    type Value = Vec<T::Value>;

    fn align(&self, cfg: &Config) -> usize {
        T::default().align(cfg)
    }

    fn id(&self) -> u64 {
        self.clone().into_dynamic().id()
    }

    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let value = self.clone().into_dynamic().load(cfg, src)?;
        Ok(value.into_items())
    }
}

impl<T: SizedType + UnitType> UnitType for StaticVectorType<T>
where
    T::Value: SizedValue + Default + Clone,
{}

impl<V: SizedValue> Value for Vec<V>
where
    V: Default + Clone,
    V::Type: SizedType + UnitType,
{
    type Type = StaticVectorType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        vector_size(&self.type_().into_dynamic(), self.len(), cfg)
    }

    fn type_(&self) -> Self::Type {
        Self::Type::default()
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        VectorValue::from_items(V::Type::default(), self.clone()).unwrap().store(cfg, dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        let vec: Vec<i32> = vec![1, 2, 3];
        let dvec = VectorValue::from_items(I32Type, vec.clone()).unwrap();
        assert_eq!(vec.type_().id(), dvec.type_().id())
    }
}
