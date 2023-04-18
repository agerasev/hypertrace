use crate::{
    io::{CntRead, CntWrite},
    source::SourceTree,
    Config, Entity, EntityId, EntitySource, SizedEntity,
};
use std::{io, marker::PhantomData};

// Unit

impl EntityId for () {
    fn name() -> (String, String) {
        ("void".into(), "empty".into())
    }
}

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

    fn min_size(_: &Config) -> usize {
        0
    }

    fn is_dyn_sized() -> bool {
        false
    }

    fn store<W: CntWrite>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl SizedEntity for () {
    fn static_size(_: &Config) -> usize {
        0
    }
}

impl EntitySource for () {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("types.hh")
    }
}

// PhantomData

impl<T: 'static> EntityId for PhantomData<T> {
    fn name() -> (String, String) {
        ("void".into(), "phantom".into())
    }
}

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

    fn min_size(_: &Config) -> usize {
        0
    }

    fn is_dyn_sized() -> bool {
        false
    }

    fn store<W: CntWrite>(&self, _: &Config, _: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl<T: 'static> SizedEntity for PhantomData<T> {
    fn static_size(_: &Config) -> usize {
        0
    }
}

impl<T: 'static> EntitySource for PhantomData<T> {
    fn source(_: &Config) -> SourceTree {
        SourceTree::new("types.hh")
    }
}
