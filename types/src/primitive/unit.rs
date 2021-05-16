use crate::{
    io::{CntRead, CntWrite},
    source::SourceTree,
    Config, Entity, EntityId, SizedEntity, EntitySource,
};
use std::{io, marker::PhantomData};

// Unit

impl EntityId for () {
    fn name() -> String {
        String::from("void")
    }

    fn data_prefix() -> String {
        String::from("empty")
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
    fn type_size(_: &Config) -> usize {
        0
    }
}

impl EntitySource for () {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("types.hh")
    }
}

// PhantomData

impl<T: 'static> EntityId for PhantomData<T> {
    fn name() -> String {
        String::from("void")
    }

    fn data_prefix() -> String {
        String::from("phantom")
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
    fn type_size(_: &Config) -> usize {
        0
    }
}

impl<T: 'static> EntitySource for PhantomData<T> {
    fn data_source(_: &Config) -> SourceTree {
        SourceTree::new("types.hh")
    }
}
