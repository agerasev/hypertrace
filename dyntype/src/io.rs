use crate::{traits::*, utils::*, Config};
use std::io::{self, Read, Write};

pub trait CountingRead: Read {
    fn position(&self) -> usize;
    fn align(&mut self, align: usize) -> io::Result<()> {
        let mut pos = self.position();
        let mut buf = [0];
        while pos % align != 0 {
            self.read_exact(&mut buf)?;
            pos += 1;
        }
        Ok(())
    }
}
impl<'a, R: CountingRead + ?Sized> CountingRead for &'a mut R {
    fn position(&self) -> usize {
        (**self).position()
    }
}

pub trait CountingWrite: Write {
    fn position(&self) -> usize;
    fn align(&mut self, align: usize) -> io::Result<()> {
        let mut pos = self.position();
        while pos % align != 0 {
            self.write_all(&[0])?;
            pos += 1;
        }
        Ok(())
    }
}
impl<'a, W: CountingWrite + ?Sized> CountingWrite for &'a mut W {
    fn position(&self) -> usize {
        (**self).position()
    }
}

pub struct ReadWrapper<R: Read> {
    inner: R,
    pos: usize,
}
impl<R: Read> ReadWrapper<R> {
    pub fn new(read: R) -> Self {
        Self { inner: read, pos: 0 }
    }
}
impl<R: Read> Read for ReadWrapper<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.pos += n;
        Ok(n)
    }
}
impl<R: Read> CountingRead for ReadWrapper<R> {
    fn position(&self) -> usize {
        self.pos
    }
}

pub struct WriteWrapper<W: Write> {
    inner: W,
    pos: usize,
}
impl<W: Write> WriteWrapper<W> {
    pub fn new(write: W) -> Self {
        Self { inner: write, pos: 0 }
    }
}
impl<W: Write> Write for WriteWrapper<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.pos += n;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
impl<W: Write> CountingWrite for WriteWrapper<W> {
    fn position(&self) -> usize {
        self.pos
    }
}

pub trait ValueReader {
    fn read_value<T: Type>(&mut self, cfg: &Config, type_: &T) -> io::Result<T::Value>;
}
impl<R: CountingRead> ValueReader for R {
    fn read_value<T: Type>(&mut self, cfg: &Config, type_: &T) -> io::Result<T::Value> {
        let align = type_.align(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_, align);
        assert!(
            is_pow2(type_.align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_,
            align
        );
        self.align(align)?;

        let pos = self.position();
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
impl<W: CountingWrite> ValueWriter for W {
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
        self.align(align)?;

        let pos = self.position();
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
