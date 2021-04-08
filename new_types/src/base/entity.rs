use crate::{
    hash::DefaultHasher,
    io::{CntRead, CntWrite},
    Config, SourceInfo,
};
use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    io,
};

/// Static entity type.
pub trait Entity: 'static + Sized {
    /// Align of type.
    fn align(cfg: &Config) -> usize;

    /// Type identifier.
    fn type_id() -> u64 {
        let mut hasher = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }

    /// Loads the instance of type.
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self>;

    /// Size of instance.
    fn size(&self, cfg: &Config) -> usize;

    /// Stores the instance to a writer.
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;

    /// Returns a kernel source info of the type.
    fn source(cfg: &Config) -> SourceInfo;
}

/// Sized static entity.
pub trait SizedEntity: Entity {
    /// Size of type.
    fn type_size(cfg: &Config) -> usize;
}
