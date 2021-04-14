use crate::{
    Entity, SizedEntity,
    io::{CntRead, CntWrite, EntityReader, EntityWriter},
    math, Config, SourceInfo,
};
use std::{
    io,
};

impl<T: SizedEntity> Entity for Vec<T> {
    fn align(cfg: &Config) -> usize {
        math::lcm(usize::align(cfg), T::align(cfg))
    }

    fn size(&self, cfg: &Config) -> usize {
        math::upper_multiple(
            math::upper_multiple(usize::type_size(cfg), T::align(cfg))
                + self.len() * T::type_size(cfg),
            Self::align(cfg),
        )
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

    fn entity_source(cfg: &Config) -> SourceInfo {
        let src = T::entity_source(cfg);
        SourceInfo::new(
            format!("Vector_{}", src.name),
            format!("vector_{}", src.prefix),
        )
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
