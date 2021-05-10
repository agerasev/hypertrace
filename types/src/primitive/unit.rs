use crate::{
    io::{CntRead, CntWrite},
    source::SourceTree,
    Config, Entity, Named, SizedEntity, Sourced,
};
use std::{io, marker::PhantomData};

// Unit

impl Named for () {
    fn type_name(_: &Config) -> String {
        String::from("void")
    }

    fn type_prefix(_: &Config) -> String {
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

    fn type_source(_: &Config) -> SourceTree {
        SourceTree::new("types.hh")
    }
}

impl SizedEntity for () {
    fn type_size(_: &Config) -> usize {
        0
    }
}

impl Sourced for () {
    fn source(cfg: &Config) -> SourceTree {
        Self::type_source(cfg)
    }
}

// PhantomData

impl<T: 'static> Named for PhantomData<T> {
    fn type_name(_: &Config) -> String {
        String::from("void")
    }

    fn type_prefix(_: &Config) -> String {
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

    fn type_source(_: &Config) -> SourceTree {
        SourceTree::new("types.hh")
    }
}

impl<T: 'static> SizedEntity for PhantomData<T> {
    fn type_size(_: &Config) -> usize {
        0
    }
}

impl<T: 'static> Sourced for PhantomData<T> {
    fn source(cfg: &Config) -> SourceTree {
        Self::type_source(cfg)
    }
}
