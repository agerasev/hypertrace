use crate::{config::*, containers::TypedVec, io::*, primitive::*, traits::*, utils::*};
use std::{
    hash::{Hash, Hasher},
    io,
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
    type Value = Vector<T::Value>;

    fn align(&self, cfg: &Config) -> usize {
        lcm(self.item_type.align(cfg), UsizeType.align(cfg))
    }

    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        type_id::<Self>().hash(&mut hasher);
        self.item_type.id().hash(&mut hasher);
        hasher.finish()
    }

    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let len = src.read_value(cfg, &UsizeType)?;
        src.align(self.item_type.align(cfg))?;
        let items = (0..len)
            .map(|_| src.read_value(cfg, &self.item_type))
            .collect::<Result<Vec<_>, _>>()?;
        src.align(self.align(cfg))?;
        Self::Value::from_items(self.item_type.clone(), items).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Item type mismatch while loading Vector",
            )
        })
    }
}

#[derive(Clone)]
pub struct Vector<V: SizedValue>
where
    V::Type: SizedType,
{
    inner: TypedVec<V>,
}

impl<V: SizedValue> Vector<V>
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

impl<V: SizedValue> Value for Vector<V>
where
    V::Type: SizedType,
{
    type Type = VectorType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        upper_multiple(
            upper_multiple(UsizeType.size(cfg), self.item_type().align(cfg))
                + self.items().len() * self.item_type().size(cfg),
            self.align(cfg),
        )
    }

    fn type_(&self) -> Self::Type {
        VectorType::new(self.item_type().clone())
    }

    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        dst.write_value(cfg, &self.items().len())?;
        dst.align(self.item_type().align(cfg))?;
        for item in self.items().iter() {
            dst.write_value(cfg, item)?;
        }
        dst.align(self.align(cfg))?;
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
        self.clone().into_dynamic().align(cfg)
    }

    fn id(&self) -> u64 {
        self.clone().into_dynamic().id()
    }

    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
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
        let item_type = V::Type::default();
        upper_multiple(
            upper_multiple(UsizeType.size(cfg), item_type.align(cfg))
                + self.len() * item_type.size(cfg),
            self.align(cfg),
        )
    }

    fn type_(&self) -> Self::Type {
        Self::Type::default()
    }

    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        Vector::from_items(V::Type::default(), self.clone())
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
        let dvec = Vector::<i32>::new(I32Type);
        assert_eq!(vec.type_().id(), dvec.type_().id())
    }

    #[test]
    fn store_load_static() {
        let vec: Vec<i32> = vec![1, 2];
        let mut buf = CountingWrapper::new(Vec::<u8>::new());
        buf.write_value(&CFG, &vec).unwrap();
        assert_eq!(
            vec,
            CountingWrapper::new(&buf.inner()[..])
                .read_value(&CFG, &<Vec<i32> as Value>::Type::default())
                .unwrap()
        );
    }

    #[test]
    fn store_load() {
        let vec = Vector::<i32>::from_items(I32Type, vec![1, 2]).unwrap();
        let mut buf = CountingWrapper::new(Vec::<u8>::new());
        buf.write_value(&CFG, &vec).unwrap();
        assert_eq!(
            vec.into_items(),
            CountingWrapper::new(&buf.inner()[..])
                .read_value(&CFG, &VectorType::new(I32Type))
                .unwrap()
                .into_items()
        );
    }
}
