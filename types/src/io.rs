use crate::{Entity, math, Config};
use std::{io::{self, Read, Write}, any::type_name};

pub trait CntRead: Read {
    fn position(&self) -> usize;
    fn skip(&mut self, count: usize) -> io::Result<()> {
        let mut buf = [0];
        for _ in 0..count {
            self.read_exact(&mut buf)?;
        }
        Ok(())
    }
    fn align(&mut self, align: usize) -> io::Result<()> {
        self.skip((align - (self.position() % align)) % align)
    }
}

pub trait CntWrite: Write {
    fn position(&self) -> usize;
    fn skip(&mut self, count: usize) -> io::Result<()> {
        for _ in 0..count {
            self.write_all(&[0])?;
        }
        Ok(())
    }
    fn align(&mut self, align: usize) -> io::Result<()> {
        self.skip((align - (self.position() % align)) % align)
    }
}

pub struct CntWrapper<T> {
    inner: T,
    pos: usize,
}
impl<T> CntWrapper<T> {
    pub fn new(read: T) -> Self {
        Self {
            inner: read,
            pos: 0,
        }
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    pub fn into_inner(self) -> T {
        self.inner
    }
}
impl<R: Read> Read for CntWrapper<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.pos += n;
        Ok(n)
    }
}
impl<R: Read> CntRead for CntWrapper<R> {
    fn position(&self) -> usize {
        self.pos
    }
}
impl<W: Write> Write for CntWrapper<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.pos += n;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
impl<W: Write> CntWrite for CntWrapper<W> {
    fn position(&self) -> usize {
        self.pos
    }
}

pub trait EntityReader {
    fn read_entity<T: Entity>(&mut self, cfg: &Config) -> io::Result<T>;
}
impl<R: CntRead> EntityReader for R {
    fn read_entity<T: Entity>(&mut self, cfg: &Config) -> io::Result<T> {
        let align = T::align(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_name::<T>(), align);
        assert!(
            math::is_pow2(T::align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_name::<T>(),
            align,
        );

        let pos = self.position();
        assert!(
            pos % align == 0,
            "Stream is not properly aligned (position: {}) for type {:?} (align: {})",
            pos,
            type_name::<T>(),
            align,
        );

        let value = T::load(cfg, self)?;
        let shift = self.position() - pos;
        let size = value.size(cfg);
        assert!(
            size % align == 0,
            "Align of type {:?} ({}) is not a multiple of its value size ({})",
            type_name::<T>(),
            align,
            size,
        );
        assert_eq!(
            size, shift,
            "Size of a value of type {:?} ({}) differs from a count of actually read bytes ({})",
            type_name::<T>(), size, shift,
        );
        Ok(value)
    }
}

pub trait EntityWriter {
    fn write_entity<T: Entity>(&mut self, cfg: &Config, entity: &T) -> io::Result<()>;
}
impl<W: CntWrite> EntityWriter for W {
    fn write_entity<T: Entity>(&mut self, cfg: &Config, entity: &T) -> io::Result<()> {
        let align = T::align(cfg);
        let size = entity.size(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_name::<T>(), align);
        assert!(
            math::is_pow2(T::align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_name::<T>(),
            align
        );
        assert!(
            size % align == 0,
            "Align of type {:?} ({}) is not a multiple of its value size ({})",
            type_name::<T>(),
            align,
            size
        );

        let pos = self.position();
        assert!(
            pos % align == 0,
            "Stream is not properly aligned (position: {}) for type {:?} (align: {})",
            pos,
            type_name::<T>(),
            align
        );

        entity.store(cfg, self)?;
        let shift = self.position() - pos;
        assert_eq!(
            size, shift,
            "Size of a value of type {:?} ({}) differs from a count of actually written bytes ({})",
            type_name::<T>(), size, shift
        );
        Ok(())
    }
}

#[cfg(test)]
#[derive(Clone, Default, Debug)]
pub struct TestBuffer {
    vec: Vec<u8>,
}

#[cfg(test)]
impl TestBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reader(&self) -> CntWrapper<&[u8]> {
        CntWrapper::new(self.vec.as_ref())
    }
    pub fn writer(&mut self) -> CntWrapper<&mut Vec<u8>> {
        CntWrapper::new(&mut self.vec)
    }
}
