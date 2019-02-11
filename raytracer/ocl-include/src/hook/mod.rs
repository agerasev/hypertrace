mod mem;
mod fs;

use std::io;
use std::path::Path;


pub trait Hook {
    type Handle: Handle;
    fn find(&self, dir: Option<&Path>, name: &Path) -> io::Result<Self::Handle>;
}

pub trait Handle {
    fn path(&self) -> &Path;
    fn read(self) -> io::Result<String>;
}
