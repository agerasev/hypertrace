use crate::{
    base::{Entity, SizedEntity, SizedType, SizedValue, Type, Value},
    containers::TypedVec,
    hash::DefaultHasher,
    io::{CntRead, CntWrite, ValueReader, ValueWriter},
    Config, SourceInfo,
};
use std::{hash::Hasher, io, iter};
use vecmat::Vector;

impl<T: SizedEntity, const N: usize> Entity for [T; N] {
    fn align(cfg: &Config) -> usize {
        T::align(cfg)
    }

    fn type_id() -> u64 {
        let mut hasher = DefaultHasher::default();
        hasher.write_u64(T::type_id());
        hasher.write_usize(N);
        hasher.finish()
    }

    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let mut err: Option<io::Error> = None;
        Vector::try_from_iter(iter::from_fn(|| match T::load(cfg, src) {
            Ok(x) => Some(x),
            Err(e) => {
                err = Some(e);
                None
            }
        }))
        .map(|vec| vec.into_array())
        .ok_or_else(|| err.unwrap())
    }

    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }

    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for x in self {
            x.store(cfg, dst)?;
        }
        Ok(())
    }

    fn source(cfg: &Config) -> SourceInfo {
        let src = T::source(cfg);
        SourceInfo::new(
            format!("Array_{}_{}", src.name, N),
            format!("array_{}_{}", src.prefix, N),
        )
    }
}

impl<T: SizedEntity, const N: usize> SizedEntity for [T; N] {
    fn type_size(cfg: &Config) -> usize {
        T::type_size(cfg) * N
    }
}

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
        let mut hasher = DefaultHasher::default();
        hasher.write_u64(self.item_type.id());
        hasher.write_usize(self.item_count);
        hasher.finish()
    }

    fn load<R: CntRead>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let mut value = Self::Value::new(self.item_type.clone());
        for _ in 0..self.item_count {
            value
                .items_mut()
                .push(src.read_value(cfg, &self.item_type)?)
                .map_err(|item| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Type mismatch: {:?} is expected but {:?} was provided",
                            value.item_type(),
                            item.type_of()
                        ),
                    )
                })?;
        }
        Ok(value)
    }

    fn source(&self, cfg: &Config) -> SourceInfo {
        let src = self.item_type.source(cfg);
        SourceInfo::new(
            format!("Array_{}_{}", src.name, self.item_count),
            format!("array_{}_{}", src.prefix, self.item_count),
        )
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
        self.type_of().size(cfg)
    }

    fn type_of(&self) -> Self::Type {
        ArrayType::new(self.item_type().clone(), self.items().len())
    }

    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for item in self.items().iter() {
            dst.write_value(cfg, item)?;
        }
        Ok(())
    }
}

impl<V: SizedValue> SizedValue for ArrayValue<V> where V::Type: SizedType {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        base::{EntityType, EntityValue},
        config::HOST_CONFIG,
        io::{EntityReader, EntityWriter, TestBuffer},
    };

    #[test]
    fn ids() {
        let arr: [i32; 3] = [1, 2, 3];
        let darr = ArrayValue::from_items(
            EntityType::<i32>::new(),
            arr.iter().cloned().map(EntityValue::new).collect(),
        )
        .unwrap();
        assert_eq!(<[i32; 3]>::type_id(), darr.type_of().id());
    }

    #[test]
    fn store_load() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let mut buf = TestBuffer::new();
        buf.writer().write_entity(&HOST_CONFIG, &arr).unwrap();
        assert_eq!(
            arr,
            buf.reader().read_entity::<[i32; 5]>(&HOST_CONFIG).unwrap()
        );
    }
}
