use std::io;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Hook {
	fn read(&mut self, src: &Path, dst: &Path) -> io::Result<Vec<u8>>;
}

pub struct FileHook {
	include_dirs: Vec<PathBuf>,
}

impl Hook for FileHook {

}

struct Collector<H: Hook> {
	hook: H,
	main: PathBuf,
}

impl<H: Hook> Collector<H> {
	pub fn new(hook: H) -> Self {
		Collector { hook }
	}
}
