use std::io;
use crate::{Config, Entity, io::{CountingRead, CountingWrite}, SourceTree};

impl Entity for () {
    fn align(_: &Config) -> usize {
        1
    }

    fn load<R: CountingRead + ?Sized>(_: &Config, _: &mut R) -> io::Result<Self> {
        Ok(())
    }

    fn size(&self, _: &Config) -> usize {
        0
    }

    fn store<W: CountingWrite + ?Sized>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn source(_: &Config) -> Option<SourceTree> {
        None
    }
}
