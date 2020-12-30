use crate::{traits::*, Config};
use std::io::{self, Read, Write};

pub fn write_and_align<V: Value, W: Write + ?Sized>(
    value: &V,
    cfg: &Config,
    align: usize,
    dst: &mut W,
) -> io::Result<()> {
    value.store(cfg, dst)?;
    let mut pos = value.size(cfg);
    while pos % align != 0 {
        dst.write_all(&[0])?;
        pos += 1;
    }
    Ok(())
}

pub fn read_and_align<T: Type, R: Read + ?Sized>(
    type_: &T,
    cfg: &Config,
    align: usize,
    src: &mut R,
) -> io::Result<T::Value> {
    let value = type_.load(cfg, src)?;
    let mut pos = value.size(cfg);
    let mut buf = [0];
    while pos % align != 0 {
        src.read_exact(&mut buf)?;
        pos += 1;
    }
    Ok(value)
}
