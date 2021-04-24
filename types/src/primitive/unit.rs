use crate::{
    io::{CntRead, CntWrite},
    Config, Entity, SizedEntity, SourceInfo,
};
use std::{io, marker::PhantomData};

// Unit

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

    fn entity_source(_: &Config) -> SourceInfo {
        SourceInfo::with_root("void", "empty", "types.hh")
    }
}

impl SizedEntity for () {
    fn type_size(_: &Config) -> usize {
        0
    }
}

// PhantomData

impl<T: 'static> Entity for PhantomData<T> {
    fn align(_: &Config) -> usize {
        1
    }

    fn load<R: CntRead>(_: &Config, _: &mut R) -> io::Result<Self> {
        Ok(PhantomData)
    }

    fn size(&self, _: &Config) -> usize {
        0
    }

    fn store<W: CntWrite>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }

    fn entity_source(_: &Config) -> SourceInfo {
        SourceInfo::with_root("void", "phantom", "types.hh")
    }
}

impl<T: 'static> SizedEntity for PhantomData<T> {
    fn type_size(_: &Config) -> usize {
        0
    }
}
