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

/// Identifiable entity.
pub trait EntityId: 'static + Sized {
    /// Type name and prefix.
    fn name() -> (String, String);

    /// Type unique id.
    fn id() -> u64 {
        let mut hasher = DefaultHasher::default();
        TypeId::of::<Self>().hash(&mut hasher);
        hasher.finish()
    }

    /// Type short text representation of unique id.
    fn tag() -> String {
        format!("{:08X}", Self::id() as u32)
    }
}

/// Trait to generate backend source code.
pub trait EntitySource: EntityId {
    /// Returns backend source code tree.
    fn source(cfg: &Config) -> SourceTree;
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
    fn static_size(cfg: &Config) -> usize;
}
