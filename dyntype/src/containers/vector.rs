use crate::{
    config::*,
    containers::{array::*, util::*, TypedVec},
    primitive::*,
    traits::*,
    util::*,
};
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
    pub item_type: T,
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
        let len = read_and_align(&UsizeType, cfg, self.align(cfg), src)?;
        Ok(Self::Value::from_array_value(
            ArrayType::new(self.item_type.clone(), len).load(cfg, src)?,
        ))
    }
}

#[derive(Clone)]
pub struct VectorValue<V: SizedValue>
where
    V::Type: SizedType,
{
    inner: TypedVec<V>,
}

impl<V: SizedValue> VectorValue<V>
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

    pub fn from_array_value(array_value: ArrayValue<V>) -> Self {
        Self::from_items(array_value.item_type().clone(), array_value.into_items()).unwrap()
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

fn vector_size<T: SizedType>(type_: &VectorType<T>, len: usize, cfg: &Config) -> usize
where
    T::Value: SizedValue,
{
    upper_multiple(UsizeType.size(cfg), type_.align(cfg)) + len * type_.item_type.size(cfg)
}

impl<V: SizedValue> Value for VectorValue<V>
where
    V::Type: SizedType,
{
    type Type = VectorType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        vector_size(&self.type_(), self.items().len(), cfg)
    }

    fn type_(&self) -> Self::Type {
        VectorType::new(self.item_type().clone())
    }

    fn store<W: Write + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        write_and_align(&self.items().len(), cfg, self.type_().align(cfg), dst)?;
        for item in self.items().iter() {
            item.store(cfg, dst)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct StaticVectorType<T: SizedType + UnitType>
where
    T::Value: SizedValue + UnitValue + Clone,
{
    phantom: PhantomData<T>,
}

impl<T: SizedType + UnitType> StaticVectorType<T>
where
    T::Value: SizedValue + UnitValue + Clone,
{
    pub fn into_dynamic(self) -> VectorType<T> {
        VectorType::new(T::default())
    }
}

impl<T: SizedType + UnitType> Type for StaticVectorType<T>
where
    T::Value: SizedValue + UnitValue + Clone,
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

impl<T: SizedType + UnitType> UnitType for StaticVectorType<T> where
    T::Value: SizedValue + UnitValue + Clone
{
}

impl<V: SizedValue + UnitValue> Value for Vec<V>
where
    V: Clone,
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
        VectorValue::from_items(V::Type::default(), self.clone())
            .unwrap()
            .store(cfg, dst)
    }
}

impl<V: SizedValue + UnitValue> UnitValue for Vec<V>
where
    V: Clone,
    V::Type: SizedType + UnitType,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    const CFG: Config = Config {
        endian: Endian::Little,
        address_width: AddressWidth::X64,
        double_support: true,
    };

    #[test]
    fn ids() {
        let vec: Vec<i32> = vec![1, 2, 3];
        let dvec = VectorValue::from_items(I32Type, vec.clone()).unwrap();
        assert_eq!(vec.type_().id(), dvec.type_().id())
    }

    #[test]
    fn store_load() {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut buf = Vec::<u8>::new();
        vec.store(&CFG, &mut buf).unwrap();
        assert_eq!(
            vec,
            <Vec<i32> as Value>::Type::default()
                .load(&CFG, &mut &buf[..])
                .unwrap()
        );
    }
}
