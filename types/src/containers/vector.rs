use crate::{
    include_template,
    io::{CntRead, CntWrite, EntityReader, EntityWriter},
    math,
    source::{SourceBuilder, SourceTree, Sourced},
    Config, Entity, Named, SizedEntity,
};
use std::io;

impl<T: SizedEntity> Named for Vec<T> {
    fn type_name(cfg: &Config) -> String {
        format!("Vector_{}", T::type_name(cfg))
    }
    fn type_prefix(cfg: &Config) -> String {
        format!("vector__{}", T::type_prefix(cfg))
    }
}

impl<T: SizedEntity> Entity for Vec<T> {
    fn align(cfg: &Config) -> usize {
        math::lcm(usize::align(cfg), T::align(cfg))
    }

    fn size(&self, cfg: &Config) -> usize {
        math::upper_multiple(
            Self::min_size(cfg) + self.len() * T::type_size(cfg),
            Self::align(cfg),
        )
    }

    fn min_size(cfg: &Config) -> usize {
        math::upper_multiple(usize::type_size(cfg), T::align(cfg))
    }

    fn is_dyn_sized() -> bool {
        true
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

    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        dst.write_entity::<usize>(cfg, &self.len())?;
        dst.align(T::align(cfg))?;
        for item in self.iter() {
            dst.write_entity(cfg, item)?;
        }
        dst.align(Self::align(cfg))?;
        Ok(())
    }

    fn type_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/vector_{}.hh", Self::type_tag()))
            .tree(T::type_source(cfg))
            .content(&include_template!(
                "container/vector.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Elem": &T::type_name(cfg),
                "elem": &T::type_prefix(cfg),
            ))
            .build()
    }
}

impl<T: SizedEntity> Sourced for Vec<T> {
    fn source(cfg: &Config) -> SourceTree {
        Self::type_source(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::HOST_CONFIG, io::TestBuffer};

    #[test]
    fn store_load() {
        let vec: Vec<i32> = vec![1, 2, 3];
        let mut buf = TestBuffer::new();
        buf.writer().write_entity(&HOST_CONFIG, &vec).unwrap();
        assert_eq!(buf.vec.len(), vec.size(&HOST_CONFIG));
        assert_eq!(
            vec,
            buf.reader().read_entity::<Vec<i32>>(&HOST_CONFIG).unwrap()
        );
    }
}
