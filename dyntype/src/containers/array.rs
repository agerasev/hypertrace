use crate::{traits::*, Config, TypedVec};
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
    pub item_type: T,
    pub item_count: usize,
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
        let mut value = Self::Value::new(self.item_type.clone());
        for _ in 0..self.item_count {
            value
                .items_mut()
                .push(self.item_type.load(cfg, src)?)
                .map_err(|item| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Type mismatch: {:?} is expected but {:?} was provided",
                            value.item_type(),
                            item.type_()
                        ),
                    )
                })?;
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

#[derive(Clone)]
pub struct ArrayValue<V: SizedValue>
where
    V::Type: SizedType,
{
    inner: TypedVec<V>,
}

impl<V: SizedValue> ArrayValue<V>
where
    V::Type: SizedType,
{
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

impl<V: SizedValue> Value for ArrayValue<V>
where
    V::Type: SizedType,
{
    type Type = ArrayType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        self.type_().size(cfg)
    }

    fn type_(&self) -> Self::Type {
        ArrayType::new(self.item_type().clone(), self.items().len())
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for item in self.items().iter() {
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

impl<T: SizedType + UnitType, const N: usize> UnitType for StaticArrayType<T, N> where
    T::Value: SizedValue + Default + Clone
{
}

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
        ArrayValue::from_items(V::Type::default(), self.clone().into())
            .unwrap()
            .store(cfg, dst)
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
        let darr = ArrayValue::from_items(i32::default().type_(), arr.into()).unwrap();
        assert_eq!(arr.type_().id(), darr.type_().id())
    }
}
