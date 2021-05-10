use crate::{
    hash::DefaultHasher,
    include_template,
    io::{CntRead, CntWrite},
    source::{SourceBuilder, SourceTree},
    Config, Entity, Named, SizedEntity, Sourced,
};
use std::{hash::Hasher, io, iter};
use vecmat::Vector;

impl<T: SizedEntity, const N: usize> Named for [T; N] {
    fn type_name(cfg: &Config) -> String {
        format!("Array_{}_{}", T::type_name(cfg), N)
    }

    fn type_prefix(cfg: &Config) -> String {
        format!("array_{}_{}", T::type_prefix(cfg), N)
    }

    fn type_id() -> u64 {
        let mut hasher = DefaultHasher::default();
        hasher.write_u64(T::type_id());
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

    fn type_source(cfg: &Config) -> SourceTree {
        SourceBuilder::new(format!("generated/array_{}.hh", Self::type_tag()))
            .tree(T::type_source(cfg))
            .content(&include_template!(
                "container/array.inl",
                "Self": &Self::type_name(cfg),
                "self": &Self::type_prefix(cfg),
                "Elem": &T::type_name(cfg),
                "elem": &T::type_prefix(cfg),
                "Num": &format!("{}", N),
            ))
            .build()
    }
}

impl<T: SizedEntity, const N: usize> SizedEntity for [T; N] {
    fn type_size(cfg: &Config) -> usize {
        T::type_size(cfg) * N
    }
}

impl<T: SizedEntity, const N: usize> Sourced for [T; N] {
    fn source(cfg: &Config) -> SourceTree {
        Self::type_source(cfg)
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
