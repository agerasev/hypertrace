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

/// Something that could be named.
pub trait Named: 'static + Sized {
    /// Type name on backend.
    fn name(cfg: &Config) -> String;

    /// Prefix for type method (usually lower-case version of the name).
    fn prefix(cfg: &Config) -> String;
}

/// Static entity type.
pub trait Entity: Named {
    /// Type identifier.
    fn type_id() -> u64 {
        let mut hasher = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }

    /// Type short text identifier.
    fn type_tag() -> String {
        format!("{:08X}", Self::type_id() as u32)
    }

    /// Align of type.
    fn align(cfg: &Config) -> usize;

    /// Size of the instance.
    fn size(&self, cfg: &Config) -> usize;

    /// Loads the instance of the type.
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self>;

    /// Stores the instance to a writer.
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;

    /// Returns a kernel source info of the type.
    fn entity_source(cfg: &Config) -> SourceInfo;
}

/// Sized static entity.
pub trait SizedEntity: Entity {
    /// Size of type.
    fn type_size(cfg: &Config) -> usize;
}
