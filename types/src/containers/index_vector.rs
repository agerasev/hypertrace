use crate::{
    include_template,
    io::{CntRead, CntWrite, EntityReader, EntityWriter},
    math,
    source::{SourceBuilder, SourceTree},
    Config, Entity, EntityId, EntitySource, SizedEntity,
};
use std::io;

#[derive(Clone, Debug)]
pub struct IndexVector<T: Entity> {
    items: Vec<T>,
}

impl<T: Entity> EntityId for IndexVector<T> {
    fn name() -> (String, String) {
        (
            format!("IndexVector_{}", T::name().0),
            format!("index_vector_{}", T::name().1),
        )
    }
}

impl<T: Entity> IndexVector<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from_items(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }

    pub fn into_items(self) -> Vec<T> {
        self.items
    }
}

impl<T: Entity> Default for IndexVector<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Entity> Entity for IndexVector<T> {
    fn align(cfg: &Config) -> usize {
        math::lcm(T::align(cfg), usize::align(cfg))
    }

    fn size(&self, cfg: &Config) -> usize {
        math::upper_multiple(
            math::upper_multiple(
                usize::static_size(cfg) * (1 + self.items().len()),
                T::align(cfg),
            ) + self.items().iter().fold(0, |st, item| st + item.size(cfg)),
            Self::align(cfg),
        )
    }

    fn min_size(cfg: &Config) -> usize {
        math::upper_multiple(usize::static_size(cfg), T::align(cfg))
    }

    fn is_dyn_sized() -> bool {
        true
    }

    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self> {
        let len = src.read_entity::<usize>(cfg)?;
        let poss = (0..len)
            .map(|_| src.read_entity::<usize>(cfg))
            .collect::<Result<Vec<_>, _>>()?;
        src.align(T::align(cfg))?;
        let sp = src.position();
        let items = poss
            .into_iter()
            .map(|vp| {
                let p = src.position();
                let ap = sp + vp;
                if p <= ap {
                    src.skip(ap - p)?;
                    src.read_entity::<T>(cfg)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "IndexVector indices are overlapping or not monotonic",
                    ))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        src.align(Self::align(cfg))?;
        Ok(Self::from_items(items))
    }

    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        dst.write_entity::<usize>(cfg, &self.items().len())?;
        for pos in self.items().iter().scan(0, |st, item| {
            let prev = *st;
            *st += item.size(cfg);
            Some(prev)
        }) {
            dst.write_entity::<usize>(cfg, &pos)?;
        }
        dst.align(T::align(cfg))?;
        for item in self.items().iter() {
            dst.write_entity::<T>(cfg, item)?;
        }
        dst.align(Self::align(cfg))?;
        Ok(())
    }
}

impl<T: Entity> EntitySource for IndexVector<T> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/{}.hh", Self::name().1))
            .tree(T::source(cfg))
            .content(&include_template!(
                "container/index_vector.inl",
                ("Self", "self") => Self::name(),
                ("Elem", "elem") => T::name(),
            ))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::HOST_CONFIG, io::TestBuffer};

    #[test]
    fn store_load() {
        let ivec = IndexVector::from_items(vec![vec![1], vec![2, 3], vec![], vec![4, 5, 6]]);
        let mut buf = TestBuffer::new();
        buf.writer().write_entity(&HOST_CONFIG, &ivec).unwrap();
        assert_eq!(buf.vec.len(), ivec.size(&HOST_CONFIG));

        let ovec = buf
            .reader()
            .read_entity::<IndexVector<Vec<i32>>>(&HOST_CONFIG)
            .unwrap();
        assert_eq!(ivec.into_items(), ovec.into_items());
    }
}
