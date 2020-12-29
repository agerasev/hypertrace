use crate::{traits::*, Config};
use std::{
    hash::{Hash, Hasher},
    io::{self, Read, Write},
    marker::PhantomData,
};
use vecmat::Vector;

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

impl<T: SizedType> Type for ArrayType<T>
where
    T::Value: SizedValue,
{
    type Value = ArrayValue<T::Value>;

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

    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let mut value = Self::Value::new();
        for _ in 0..self.item_count {
            value
                .push(self.item_type.load(cfg, src)?)
                .map_err(|_| ())
                .unwrap();
        }
        Ok(value)
    }
}

impl<T: SizedType> SizedType for ArrayType<T>
where
    T::Value: SizedValue,
{
    fn size(&self, cfg: &Config) -> usize {
        self.item_count * self.item_type.size(cfg)
    }
}

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

    pub fn from_array<const N: usize>(arr: [V; N]) -> Self where V: Default {
        Self {
            item_type: Some(V::default().type_()),
            items: Vec::from(arr),
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

    pub fn into_items(self) -> Vec<V> {
        self.items
    }
}

impl<V: SizedValue> Value for ArrayValue<V>
where
    V::Type: SizedType,
{
    type Type = ArrayType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        self.type_().size(cfg)
    }

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

#[derive(Clone, Debug, Default)]
pub struct StaticArrayType<T: SizedType + UnitType, const N: usize>
where
    T::Value: SizedValue + Default + Clone,
{
    phantom: PhantomData<T>,
}

impl<T: SizedType + UnitType, const N: usize> StaticArrayType<T, N>
where
    T::Value: SizedValue + Default + Clone,
{
    pub fn into_dynamic(self) -> ArrayType<T> {
        ArrayType::new(T::default(), N)
    }
}

impl<T: SizedType + UnitType, const N: usize> Type for StaticArrayType<T, N>
where
    T::Value: SizedValue + Default + Clone,
{
    type Value = [T::Value; N];

    fn align(&self, cfg: &Config) -> usize {
        T::default().align(cfg)
    }

    fn id(&self) -> u64 {
        self.clone().into_dynamic().id()
    }

    fn load<R: Read + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let value = self.clone().into_dynamic().load(cfg, src)?;
        Ok(Vector::try_from_iter(value.into_items().into_iter())
            .unwrap()
            .into_array())
    }
}

impl<T: SizedType + UnitType, const N: usize> SizedType for StaticArrayType<T, N>
where
    T::Value: SizedValue + Default + Clone,
{
    fn size(&self, cfg: &Config) -> usize {
        N * T::default().size(cfg)
    }
}

impl<T: SizedType + UnitType, const N: usize> UnitType for StaticArrayType<T, N>
where
    T::Value: SizedValue + Default + Clone,
{}

impl<V: SizedValue, const N: usize> Value for [V; N]
where
    V: Default + Clone,
    V::Type: SizedType + UnitType,
{
    type Type = StaticArrayType<V::Type, N>;

    fn size(&self, cfg: &Config) -> usize {
        self.type_().size(cfg)
    }

    fn type_(&self) -> Self::Type {
        Self::Type::default()
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        ArrayValue::from_array(self.clone()).store(cfg, dst)
    }
}

impl<V: SizedValue, const N: usize> SizedValue for [V; N]
where
    V: Default + Clone,
    V::Type: SizedType + UnitType,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        let arr: [i32; 3] = [1, 2, 3];
        let darr = ArrayValue::from_array(arr);
        assert_eq!(arr.type_().id(), darr.type_().id())
    }
}
