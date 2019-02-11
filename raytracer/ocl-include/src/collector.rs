use std::io;
use std::path::{Path, PathBuf};
use std::collections::{HashMap};

use regex::{Regex, RegexBuilder, Captures};

use crate::hook::Hook;


lazy_static!{
    static ref INCLUDE: Regex = RegexBuilder::new(
        "^\\s*#include\\s*([<\"])([^>\"])([>\"])\\s*$"
    ).multi_line(true).build().unwrap();
    static ref PRAGMA_ONCE: Regex = RegexBuilder::new(
        "^\\s*#pragma\\s+once\\s*$"
    ).multi_line(true).build().unwrap();
}


struct Info {
    src: Option<String>,
}

struct Collector<H: Hook> {
    hook: H,
    stack: Vec<PathBuf>,
    cache: HashMap<PathBuf, Info>,
}

impl<H: Hook> Collector<H> {
    fn new(hook: H) -> Self {
        Collector {
            hook,
            stack: Vec::new(),
            cache: HashMap::new(),
        }
    }

    fn collect(&mut self, path: &Path) -> io::Result<String> {
        let src = if self.cache.contains_key(path) {
            let info = self.cache.get(path).unwrap();
            match &info.src {
                Some(src) => {
                    if self.stack.iter().find(|other| path == other.as_path()).is_some() {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Include recursion found in '{}'", path.display()),
                        ))
                    } else {
                        src.clone()
                    }
                },
                None => return Ok(String::new()),
            }
        } else {
            let src = self.hook.read(path)?;

            let mut once = false;
            let src = PRAGMA_ONCE.replace_all(&src, |_caps: &Captures| {
                once = true;
                ""
            }).into_owned();
            
            assert!(self.cache.insert(
                path.to_path_buf(),
                if once {
                    Info { src: None }
                } else {
                    Info { src: Some(src.clone()) }
                }
            ).is_none());

            src
        };
        
        self.stack.push(path.to_path_buf());

        let mut func = |caps: &Captures| -> io::Result<String> {
            let (dir, name) = if &caps[1] == "\"" && &caps[3] == "\"" {
                let parent = match path.parent() {
                    Some(p) => p,
                    None => return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Cannot get parent of '{}'", path.display()),
                    )),
                };
                (Some(parent), PathBuf::from(&caps[2]))
            } else if &caps[1] == "<" && &caps[3] == ">" {
                (None, PathBuf::from(&caps[2]))
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid include directive found in '{}'", path.display()),
                ))
            };

            self.hook.find(dir, name.as_path()).and_then(|p| {
                self.collect(p.as_path())
            })
        };

        let mut error = None;
        let src = INCLUDE.replace_all(&src, |caps: &Captures| {
            if error.is_some() {
                return String::new();
            }
            match func(caps) {
                Ok(src) => src,
                Err(err) => {
                    error = Some(err);
                    String::new()
                },
            }
        }).into_owned();

        self.stack.pop();

        match error {
            Some(err) => return Err(err),
            None => (),
        };

        Ok(src)
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn fs_hook() {
        let hook = FsHook(["abc", "cde"]);
    }
}
