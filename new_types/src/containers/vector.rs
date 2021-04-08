use crate::{
    base::{Entity, EntityType, SizedEntity, SizedType, SizedValue, Type, Value},
    containers::TypedVec,
    hash::DefaultHasher,
    io::{CntRead, CntWrite, EntityReader, EntityWriter, ValueReader, ValueWriter},
    math, Config, SourceInfo,
};
use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    io,
};

struct VectorId;

impl<T: SizedEntity> Entity for Vec<T> {
    fn align(cfg: &Config) -> usize {
        math::lcm(usize::align(cfg), T::align(cfg))
    }

    fn type_id() -> u64 {
        let mut hasher = DefaultHasher::default();
        TypeId::of::<VectorId>().hash(&mut hasher);
        hasher.write_u64(T::type_id());
        hasher.finish()
    }

    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let len = src.read_entity::<usize>(cfg)?;
        src.align(T::align(cfg))?;
        let items = (0..len)
            .map(|_| src.read_entity::<T>(cfg))
            .collect::<Result<Vec<_>, _>>()?;
        src.align(Self::align(cfg))?;
        Ok(items)
    }

    fn size(&self, cfg: &Config) -> usize {
        math::upper_multiple(
            math::upper_multiple(usize::type_size(cfg), T::align(cfg))
                + self.len() * T::type_size(cfg),
            Self::align(cfg),
        )
    }

    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        dst.write_entity::<usize>(cfg, &self.len())?;
        dst.align(T::align(cfg))?;
        for item in self.iter() {
            dst.write_entity(cfg, item)?;
        }
        dst.align(Self::align(cfg))?;
        Ok(())
    }

    fn source(cfg: &Config) -> SourceInfo {
        VectorType::new(EntityType::<T>::new()).source(cfg)
    }
}

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
        math::lcm(self.item_type.align(cfg), usize::align(cfg))
    }

    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        TypeId::of::<VectorId>().hash(&mut hasher);
        hasher.write_u64(self.item_type.id());
        hasher.finish()
    }

    fn load<R: CntRead>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let len = src.read_entity::<usize>(cfg)?;
        src.align(self.item_type.align(cfg))?;
        let items = (0..len)
            .map(|_| src.read_value(cfg, &self.item_type))
            .collect::<Result<Vec<_>, _>>()?;
        src.align(self.align(cfg))?;
        Self::Value::from_items(self.item_type.clone(), items).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Item type mismatch while loading VectorValue",
            )
        })
    }

    fn source(&self, cfg: &Config) -> SourceInfo {
        let src = self.item_type.source(cfg);
        SourceInfo::new(
            format!("Vector_{}", src.name),
            format!("vector_{}", src.prefix),
        )
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

impl<V: SizedValue> Value for VectorValue<V>
where
    V::Type: SizedType,
{
    type Type = VectorType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        math::upper_multiple(
            math::upper_multiple(usize::type_size(cfg), self.item_type().align(cfg))
                + self.items().len() * self.item_type().size(cfg),
            self.align(cfg),
        )
    }

    fn type_of(&self) -> Self::Type {
        VectorType::new(self.item_type().clone())
    }

    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        dst.write_entity::<usize>(cfg, &self.items().len())?;
        dst.align(self.item_type().align(cfg))?;
        for item in self.items().iter() {
            dst.write_value(cfg, item)?;
        }
        dst.align(self.align(cfg))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{base::EntityValue, config::HOST_CONFIG, io::TestBuffer};

    #[test]
    fn ids() {
        let dvec = VectorValue::<EntityValue<i32>>::new(EntityType::<i32>::new());
        assert_eq!(Vec::<i32>::type_id(), dvec.type_of().id())
    }

    #[test]
    fn store_load_static() {
        let vec: Vec<i32> = vec![1, 2, 3];
        let mut buf = TestBuffer::new();
        buf.writer().write_entity(&HOST_CONFIG, &vec).unwrap();
        assert_eq!(
            vec,
            buf.reader().read_entity::<Vec<i32>>(&HOST_CONFIG).unwrap()
        );
    }

    #[test]
    fn store_load_dynamic() {
        let vec = VectorValue::<EntityValue<i32>>::from_items(
            EntityType::<i32>::new(),
            vec![1, 2, 3].into_iter().map(EntityValue::new).collect(),
        )
        .unwrap();
        let mut buf = TestBuffer::new();
        buf.writer().write_value(&HOST_CONFIG, &vec).unwrap();
        assert_eq!(
            vec.into_items()
                .into_iter()
                .map(EntityValue::into_entity)
                .collect::<Vec<_>>(),
            buf.reader()
                .read_value(&HOST_CONFIG, &VectorType::new(EntityType::<i32>::new()))
                .unwrap()
                .into_items()
                .into_iter()
                .map(EntityValue::into_entity)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn store_dynamic_load_static() {
        let vec = VectorValue::<EntityValue<i32>>::from_items(
            EntityType::<i32>::new(),
            vec![1, 2, 3].into_iter().map(EntityValue::new).collect(),
        )
        .unwrap();
        let mut buf = TestBuffer::new();
        buf.writer().write_value(&HOST_CONFIG, &vec).unwrap();
        assert_eq!(
            vec![1, 2, 3],
            buf.reader().read_entity::<Vec<i32>>(&HOST_CONFIG).unwrap()
        );
    }
}
