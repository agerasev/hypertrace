use std::io;
use std::path::{Path, PathBuf};
use std::collections::{HashMap};

use super::{Hook, Handle};

pub struct MemHook {
    files: HashMap<PathBuf, String>,
}

pub struct MemHandle {
    path: PathBuf,
    data: String,
}

impl MemHook {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn add(&mut self, path: &Path, data: String) {
        self.files.insert(path.to_path_buf(), data);
    }
}

impl Hook for MemHook {
    type Handle = MemHandle;
    fn find(&self, dir: Option<&Path>, name: &Path) -> io::Result<Self::Handle> {
        match self.files.get(name) {
            Some(data) => Ok(MemHandle {
                path: name.to_path_buf(),
                data: data.clone(),
            }),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File '{}' not found", name.display()),
            )),
        }
    }
}

impl Handle for MemHandle {
    fn path(&self) -> &Path {
        self.path.as_path()
    }

    fn read(self) -> io::Result<String> {
        Ok(self.data)
    }
}
