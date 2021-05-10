use crate::{
    hash::DefaultHasher,
    io::{CntRead, CntWrite},
    source::SourceTree,
    Config,
};
use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    io,
};

/// Something that could be identified.
pub trait Named: 'static + Sized {
    /// Type name on backend.
    fn type_name(cfg: &Config) -> String;

    /// Prefix for type method (usually lower-case version of the name).
    fn type_prefix(cfg: &Config) -> String;

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
}

/// Static entity type.
pub trait Entity: Named + Sourced {
    /// Align of type.
    fn align(cfg: &Config) -> usize;

    /// Dynamic size of the instance.
    /// *If statically-sized then it must be equal to static size.*
    fn size(&self, cfg: &Config) -> usize;

    /// Lower bound of dynamic size.
    /// *If statically-sized then it must be equal to static size.*
    fn min_size(cfg: &Config) -> usize;

    /// Checks whether the type is dynamically-sized.
    fn is_dyn_sized() -> bool;

    /// Loads the instance of the type.
    fn load<R: CntRead>(cfg: &Config, src: &mut R) -> io::Result<Self>;

    /// Stores the instance to a writer.
    fn store<W: CntWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;

    /// Returns a kernel source tree of the type.
    fn type_source(cfg: &Config) -> SourceTree;
}

/// Sized static entity.
pub trait SizedEntity: Entity {
    /// Static size of type.
    fn type_size(cfg: &Config) -> usize;
}

/// Trait to generate backend source code.
pub trait Sourced: Named {
    /// Returns backend source code tree.
    fn source(cfg: &Config) -> SourceTree;
}
