use crate::{
    Entity, SizedEntity,
    hash::DefaultHasher,
    io::{CntRead, CntWrite},
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
