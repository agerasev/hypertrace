use crate::{
    io::{CntRead, CntWrite},
    Config, Entity, SizedEntity, SourceInfo,
};
use std::io;

impl Entity for () {
    fn align(_: &Config) -> usize {
        1
    }

    fn load<R: CntRead>(_: &Config, _: &mut R) -> io::Result<Self> {
        Ok(())
    }

    fn size(&self, _: &Config) -> usize {
        0
    }

    fn store<W: CntWrite>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn source(_: &Config) -> SourceInfo {
        SourceInfo::new("void".into(), "empty".into())
    }
}

impl SizedEntity for () {
    fn type_size(_: &Config) -> usize {
        0
    }
}
