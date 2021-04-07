use crate::{
    io::{CountingRead, CountingWrite},
    Config, SourceTree,
};
use std::io;

/// Static entity type.
pub trait Entity: 'static + Sized {
    /// Align of type.
    fn align(cfg: &Config) -> usize;

    /// Loads the instance of type.
    fn load<R: CountingRead + ?Sized>(cfg: &Config, src: &mut R) -> io::Result<Self>;

    /// Size of instance.
    fn size(&self, cfg: &Config) -> usize;

    /// Stores the instance to a writer.
    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()>;

    /// Returns a source tree of the type.
    fn source(cfg: &Config) -> Option<SourceTree>;
}

/// Sized static entity.
pub trait SizedEntity: Entity {
    /// Size of type.
    fn type_size(cfg: &Config) -> usize;
}
