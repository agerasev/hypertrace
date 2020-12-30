use crate::{traits::*, utils::*, Config};
use std::io::{self, Read, Seek, SeekFrom, Write};

pub struct AlignedWriter<'a, W: Write + Seek + ?Sized> {
    stream: &'a mut W,
}

pub struct AlignedReader<'a, R: Read + Seek + ?Sized> {
    stream: &'a mut R,
}

impl<'a, W: Write + Seek + ?Sized> AlignedWriter<'a, W> {
    pub fn position(&mut self) -> usize {
        self.stream.seek(SeekFrom::Current(0)).unwrap() as usize
    }
    pub fn move_to(&mut self, pos: usize) {
        self.stream.seek(SeekFrom::Start(pos as u64)).unwrap();
    }
    pub fn make_aligned(&mut self, align: usize) -> io::Result<()> {
        let mut pos = self.position();
        while pos % align != 0 {
            self.stream.write_all(&[0])?;
            pos += 1;
        }
        Ok(())
    }
    pub fn write_value<V: Value>(&mut self, cfg: &Config, value: &V) -> io::Result<()> {
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
        value.store(cfg, self.stream)?;
        let shift = self.position() - pos;
        assert_eq!(
            size, shift,
            "Size of a value of type {:?} ({}) differs from a count of actually written bytes ({})",
            type_, size, shift
        );
        Ok(())
    }
}

impl<'a, R: Read + Seek + ?Sized> AlignedReader<'a, R> {
    pub fn position(&mut self) -> usize {
        self.stream.seek(SeekFrom::Current(0)).unwrap() as usize
    }
    pub fn move_to(&mut self, pos: usize) {
        self.stream.seek(SeekFrom::Start(pos as u64)).unwrap();
    }
    pub fn make_aligned(&mut self, align: usize) -> io::Result<()> {
        let mut pos = self.position();
        let mut buf = [0];
        while pos % align != 0 {
            self.stream.read_exact(&mut buf)?;
            pos += 1;
        }
        Ok(())
    }
    pub fn read_value<T: Type>(&mut self, cfg: &Config, type_: &T) -> io::Result<T::Value> {
        let align = type_.align(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_, align);
        assert!(
            is_pow2(type_.align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_,
            align
        );

        let pos = self.position();
        let value = type_.load(cfg, self.stream)?;
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
