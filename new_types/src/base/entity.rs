use crate::{
    io::{CountingRead, CountingWrite},
    type_id, Config, SourceInfo,
};
use std::io;

/// Static entity type.
pub trait Entity: 'static + Sized {
    /// Align of type.
    fn align(cfg: &Config) -> usize;

    /// Type identifier.
    fn type_id(_cfg: &Config) -> u64 {
        type_id::<Self>()
    }

    /// Loads the instance of type.
    fn load<R: CountingRead>(cfg: &Config, src: &mut R) -> io::Result<Self>;

    /// Size of instance.
    fn size(&self, cfg: &Config) -> usize;

    /// Stores the instance to a writer.
    fn store<W: CountingWrite>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;

    /// Returns a kernel source info of the type.
    fn source(cfg: &Config) -> SourceInfo;
}

/// Sized static entity.
pub trait SizedEntity: Entity {
    /// Size of type.
    fn type_size(cfg: &Config) -> usize;
}
