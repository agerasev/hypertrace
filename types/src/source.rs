use std::collections::{btree_map::{BTreeMap, Entry}, HashSet};
use crate::Config;
use uni_path::{Path, PathBuf};

pub trait Sourced {
    fn source(cfg: &Config) -> SourceInfo;
}

pub struct ContentMismatchError {
    pub path: PathBuf,
}

impl From<ContentMismatchError> for String {
    fn from(err: ContentMismatchError) -> String {
        format!("Files '{}' has the same path but different content", err.path)
    }
}

#[derive(Clone)]
pub struct SourceInfo {
    pub name: String,
    pub prefix: String,
    pub tree: SourceTree,
}

#[derive(Clone, Default)]
pub struct SourceTree {
    root: Option<PathBuf>,
    files: BTreeMap<PathBuf, String>,
}

impl SourceInfo {
    pub fn new(name: String, prefix: String) -> Self {
        Self {
            name,
            prefix,
            tree: SourceTree::new(),
        }
    }
}

impl SourceTree {
    pub fn new() -> Self {
        Self {
            root: None,
            files: BTreeMap::new(),
        }
    }

    pub fn root(&self) -> Option<PathBuf> {
        self.root.clone()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&Path, &String)> {
        self.files.iter().map(|(x, y)| (x.as_path(), y))
    }

    pub fn insert(&mut self, name: PathBuf, content: String) -> Result<(), ContentMismatchError> {
        match self.files.entry(name) {
            Entry::Vacant(ve) => {
                ve.insert(content);
                Ok(())
            },
            Entry::Occupied(oe) => {
                if oe.get() == &content {
                    Ok(())
                } else {
                    Err(ContentMismatchError { path: oe.key().clone() })
                }
            }
        }
    }

    pub fn append(&mut self, mut other: SourceTree) -> Result<(), ContentMismatchError> {
        let self_keys = self.files.keys().collect::<HashSet<_>>();
        let other_keys = other.files.keys().collect::<HashSet<_>>();
        let collision = self_keys.intersection(&other_keys).copied();
        for path in collision {
            if self.files.get(path) != other.files.get(path) {
                return Err(ContentMismatchError { path: path.clone() });
            }
        }
        self.files.append(&mut other.files);
        Ok(())
    }

    pub fn set_root(&mut self, root: Option<PathBuf>) {
        self.root = root;
    }
}

impl IntoIterator for SourceTree {
    type Item = (PathBuf, String);
    type IntoIter = <BTreeMap<PathBuf, String> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}
