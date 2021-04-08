use crate::{
    io::{CountingRead, CountingWrite},
    Config, Entity, SizedEntity, SourceTree,
};
use std::io;

impl Entity for () {
    fn align(_: &Config) -> usize {
        1
    }

    fn load<R: CountingRead>(_: &Config, _: &mut R) -> io::Result<Self> {
        Ok(())
    }

    fn size(&self, _: &Config) -> usize {
        0
    }

    fn store<W: CountingWrite>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn source(_: &Config) -> Option<SourceTree> {
        None
    }
}

impl SizedEntity for () {
    fn type_size(_: &Config) -> usize {
        0
    }
}
