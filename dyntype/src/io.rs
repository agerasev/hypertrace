use crate::{traits::*, utils::*, Config};
use std::io::{self, Read, Write};

pub trait CountingRead: Read {
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
    fn as_dyn_ref(&self) -> &dyn CountingRead;
    fn as_dyn_mut(&mut self) -> &mut dyn CountingRead;
}

pub trait CountingWrite: Write {
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
    fn as_dyn_ref(&self) -> &dyn CountingWrite;
    fn as_dyn_mut(&mut self) -> &mut dyn CountingWrite;
}

pub struct CountingWrapper<T> {
    inner: T,
    pos: usize,
}
impl<T> CountingWrapper<T> {
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
impl<R: Read> Read for CountingWrapper<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.pos += n;
        Ok(n)
    }
}
impl<R: Read> CountingRead for CountingWrapper<R> {
    fn position(&self) -> usize {
        self.pos
    }
    fn as_dyn_ref(&self) -> &dyn CountingRead {
        self
    }
    fn as_dyn_mut(&mut self) -> &mut dyn CountingRead {
        self
    }
}
impl<W: Write> Write for CountingWrapper<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.pos += n;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
impl<W: Write> CountingWrite for CountingWrapper<W> {
    fn position(&self) -> usize {
        self.pos
    }
    fn as_dyn_ref(&self) -> &dyn CountingWrite {
        self
    }
    fn as_dyn_mut(&mut self) -> &mut dyn CountingWrite {
        self
    }
}

pub trait ValueReader {
    fn read_value<T: Type>(&mut self, cfg: &Config, type_: &T) -> io::Result<T::Value>;
}
impl<R: CountingRead + ?Sized> ValueReader for R {
    fn read_value<T: Type>(&mut self, cfg: &Config, type_: &T) -> io::Result<T::Value> {
        let align = type_.align(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_, align);
        assert!(
            is_pow2(type_.align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_,
            align
        );

        let pos = self.position();
        assert!(pos % align == 0, "Stream is not properly aligned (position: {}) for type {:?} (align: {})", pos, type_, align);
        
        let value = type_.load(cfg, self)?;
        let shift = self.position() - pos;
        let size = value.size(cfg);
        assert!(
            size % align == 0,
            "Align of type {:?} ({}) is not a multiple of its value size ({})",
            type_,
            align,
            size
        );
        assert_eq!(
            size, shift,
            "Size of a value of type {:?} ({}) differs from a count of actually read bytes ({})",
            type_, size, shift
        );
        Ok(value)
    }
}

pub trait ValueWriter {
    fn write_value<V: Value>(&mut self, cfg: &Config, value: &V) -> io::Result<()>;
}
impl<W: CountingWrite + ?Sized> ValueWriter for W {
    fn write_value<V: Value>(&mut self, cfg: &Config, value: &V) -> io::Result<()> {
        let type_ = value.type_();
        let align = type_.align(cfg);
        let size = value.size(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_, align);
        assert!(
            is_pow2(type_.align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_,
            align
        );
        assert!(
            size % align == 0,
            "Align of type {:?} ({}) is not a multiple of its value size ({})",
            type_,
            align,
            size
        );

        let pos = self.position();
        assert!(pos % align == 0, "Stream is not properly aligned (position: {}) for type {:?} (align: {})", pos, type_, align);
        
        value.store(cfg, self)?;
        let shift = self.position() - pos;
        assert_eq!(
            size, shift,
            "Size of a value of type {:?} ({}) differs from a count of actually written bytes ({})",
            type_, size, shift
        );
        Ok(())
    }
}
