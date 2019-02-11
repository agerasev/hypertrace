mod mem;
mod fs;

use std::io;
use std::path::{Path, PathBuf};


pub trait Hook {
    fn find(&self, dir: Option<&Path>, name: &Path) -> io::Result<PathBuf>;
    fn read(&self, path: &Path) -> io::Result<String>;
}
