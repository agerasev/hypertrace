use std::io;
use std::fs;
use std::path::{Path, PathBuf};

use super::Hook;


/// Hook to search for included files in file system.
pub struct FsHook {
    include_dirs: Vec<PathBuf>,
}

impl FsHook {
    pub fn new(include_dirs: &[&Path]) -> Self {
        FsHook {
            include_dirs: include_dirs.iter().map(|e| e.to_path_buf()).collect(),
        }
    }

    pub fn check_dirs(&self) -> io::Result<()> {
        for dir in &self.include_dirs {
            let meta = fs::metadata(dir)?;
            if !meta.is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("'{}' is not a directory", dir.display()),
                ))
            }
        }
        Ok(())
    }

    pub fn check_file(&self, path: &Path) -> io::Result<()> {
        match fs::metadata(path) {
            Ok(meta) => {
                if !meta.is_file() {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("'{}' is not a file", path.display()),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    } 
}

impl Hook for FsHook {
    fn find(&self, dir: Option<&Path>, name: &Path) -> io::Result<PathBuf> {
        match dir {
            Some(dir) => {
                let path = dir.join(name);
                match self.check_file(&path) {
                    Ok(()) => return Ok(path),
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => (),
                        _ => return Err(e),
                    }
                }
            },
            None => (),
        }

        for dir in self.include_dirs.iter() {
            let path = dir.join(name);
            match self.check_file(&path) {
                Ok(()) => return Ok(path),
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => (),
                    _ => return Err(e),
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "File '{}' not found in any of include_dirs",
                name.display()
            ),
        ))
    }

    fn read(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }
}
