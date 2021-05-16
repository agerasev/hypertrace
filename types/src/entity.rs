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
pub trait EntityId: 'static + Sized {
    /// Type name on backend.
    fn name() -> String;

    /// Prefix for type method (usually lower-case version of the name).
    fn data_prefix() -> String;

    /// Type identifier.
    fn id() -> u64 {
        let mut hasher = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }

    /// Type short text identifier.
    fn tag() -> String {
        format!("{:08X}", Self::id() as u32)
    }
}

/// Trait to generate backend source code.
pub trait EntitySource: EntityId {
    /// Returns backend source code tree.
    fn data_source(cfg: &Config) -> SourceTree;
}

/// Static entity type.
pub trait Entity: EntityId + EntitySource {
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
}

/// Sized static entity.
pub trait SizedEntity: Entity {
    /// Static size of type.
    fn type_size(cfg: &Config) -> usize;
}
