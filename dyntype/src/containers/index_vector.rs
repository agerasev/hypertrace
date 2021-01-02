use crate::{containers::TypedVec, io::*, primitive::*, traits::*, utils::*, Config};
use std::{
    hash::{Hash, Hasher},
    io,
};

#[derive(Clone, Debug)]
pub struct IndexVectorType<T: Type> {
    item_type: T,
}

impl<T: Type> IndexVectorType<T> {
    pub fn new(item_type: T) -> Self {
        Self { item_type }
    }
}

impl<T: Type> Type for IndexVectorType<T> {
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

    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let len = src.read_value(cfg, &UsizeType)?;
        let poss = (0..len)
            .map(|_| src.read_value(cfg, &UsizeType))
            .collect::<Result<Vec<_>, _>>()?;
        src.align(self.item_type.align(cfg))?;
        let sp = src.position();
        let items = poss
            .into_iter()
            .map(|vp| {
                let p = src.position();
                let ap = sp + vp;
                if p <= ap {
                    src.skip(ap - p)?;
                    src.read_value(cfg, &self.item_type)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "IndexBuffer indices are overlapping or not monotonic",
                    ))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        src.align(self.align(cfg))?;
        Self::Value::from_items(self.item_type.clone(), items).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "IndexBuffer item type mismatch")
        })
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
    type Type = IndexVectorType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        upper_multiple(
            upper_multiple(
                UsizeType.size(cfg) * (1 + self.items().len()),
                self.item_type().align(cfg),
            ) + self.items().iter().fold(0, |st, item| st + item.size(cfg)),
            self.align(cfg),
        )
    }

    fn type_(&self) -> Self::Type {
        IndexVectorType::new(self.item_type().clone())
    }

    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        dst.write_value(cfg, &self.items().len())?;
        for pos in self.items().iter().scan(0, |st, item| {
            let prev = *st;
            *st += item.size(cfg);
            Some(prev)
        }) {
            dst.write_value(cfg, &pos)?;
        }
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
    use crate::config::*;

    #[test]
    fn store_load() {
        let ivec = IndexBuffer::from_items(
            <Vec<i32> as Value>::Type::default(),
            vec![vec![1], vec![2, 3], vec![4, 5, 6]],
        )
        .unwrap();
        let mut buf = TestBuffer::new();
        buf.writer().write_value(&HOST_CONFIG, &ivec).unwrap();
        let ovec = buf
            .reader()
            .read_value(
                &HOST_CONFIG,
                &IndexVectorType::new(<Vec<i32> as Value>::Type::default()),
            )
            .unwrap();
        assert_eq!(ivec.into_items(), ovec.into_items(),);
    }
}
