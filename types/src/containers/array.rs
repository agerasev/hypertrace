use crate::{
    hash::DefaultHasher,
    include_template,
    io::{CntRead, CntWrite},
    source::{SourceBuilder, SourceTree},
    Config, Entity, EntityId, SizedEntity, EntitySource,
};
use std::{hash::Hasher, io, iter};
use vecmat::Vector;

impl<T: SizedEntity, const N: usize> EntityId for [T; N] {
    fn name() -> String {
        format!("Array_{}_{}", T::name(), N)
    }

    fn data_prefix() -> String {
        format!("array__{}__{}", T::data_prefix(), N)
    }

    fn id() -> u64 {
        let mut hasher = DefaultHasher::default();
        hasher.write_u64(T::id());
        hasher.write_usize(N);
        hasher.finish()
    }
}

impl<T: SizedEntity, const N: usize> Entity for [T; N] {
    fn align(cfg: &Config) -> usize {
        T::align(cfg)
    }

    fn size(&self, cfg: &Config) -> usize {
        Self::type_size(cfg)
    }

    fn min_size(cfg: &Config) -> usize {
        Self::type_size(cfg)
    }

    fn is_dyn_sized() -> bool {
        false
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

    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        for x in self {
            x.store(cfg, dst)?;
        }
        Ok(())
    }
}

impl<T: SizedEntity, const N: usize> SizedEntity for [T; N] {
    fn type_size(cfg: &Config) -> usize {
        T::type_size(cfg) * N
    }
}

impl<T: SizedEntity, const N: usize> EntitySource for [T; N] {
    fn data_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/array_{}.hh", Self::tag()))
            .tree(T::data_source(cfg))
            .content(&include_template!(
                "container/array.inl",
                "Self": Self::name(),
                "self": Self::data_prefix(),
                "Element": T::name(),
                "element": T::data_prefix(),
                "size": format!("{}", N),
            ))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::HOST_CONFIG,
        io::{EntityReader, EntityWriter, TestBuffer},
    };

    #[test]
    fn store_load() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let mut buf = TestBuffer::new();
        buf.writer().write_entity(&HOST_CONFIG, &arr).unwrap();
        assert_eq!(buf.vec.len(), arr.size(&HOST_CONFIG));
        assert_eq!(
            arr,
            buf.reader().read_entity::<[i32; 5]>(&HOST_CONFIG).unwrap()
        );
    }
}
