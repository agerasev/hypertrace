use crate::{
    include_template,
    io::{CntRead, CntWrite, EntityReader, EntityWriter},
    math,
    source::{SourceBuilder, SourceTree},
    Config, Entity, EntityId, SizedEntity, EntitySource,
};
use std::io;

impl<T: SizedEntity> EntityId for Vec<T> {
    fn name() -> (String, String) {
        (
            format!("Vector_{}", T::name().0),
            format!("vector__{}", T::name().1),
        )
    }
}

impl<T: SizedEntity> Entity for Vec<T> {
    fn align(cfg: &Config) -> usize {
        math::lcm(usize::align(cfg), T::align(cfg))
    }

    fn size(&self, cfg: &Config) -> usize {
        math::upper_multiple(
            Self::min_size(cfg) + self.len() * T::static_size(cfg),
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
}

impl<T: SizedEntity> EntitySource for Vec<T> {
    fn source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/vector_{}.hh", Self::tag()))
            .tree(T::source(cfg))
            .content(&include_template!(
                "container/vector.inl",
                ("Self", "self") => Self::name(),
                ("Element", "element") => T::name(),
                "element_size" => format!("{}", T::static_size(cfg)),
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
