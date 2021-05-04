use std::collections::{btree_map::{BTreeMap, Entry}, HashSet};
use crate::Config;
use uni_path::{Path, PathBuf};

pub trait Sourced {
    fn source(cfg: &Config) -> SourceInfo;
}

#[derive(Clone, Debug)]
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
    root: PathBuf,
    files: BTreeMap<PathBuf, String>,
}

impl SourceInfo {
    pub fn new<A: Into<String>, B: Into<String>>(name: A, prefix: B, tree: SourceTree) -> Self {
        Self {
            name: name.into(),
            prefix: prefix.into(),
            tree,
        }
    }
    pub fn with_root<A: Into<String>, B: Into<String>, C: Into<PathBuf>>(name: A, prefix: B, root: C) -> Self {
        Self {
            name: name.into(),
            prefix: prefix.into(),
            tree: SourceTree::new(root.into()),
        }
    }
}

impl SourceTree {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            files: BTreeMap::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = root;
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
}

impl IntoIterator for SourceTree {
    type Item = (PathBuf, String);
    type IntoIter = <BTreeMap<PathBuf, String> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

pub struct SourceBuilder {
    tree: SourceTree,
    content: String,
}

impl SourceBuilder {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self {
            tree: SourceTree::new(root.into()),
            content: String::from("#pragma once\n"),
        }
    }
    pub fn tree(mut self, tree: SourceTree) -> Self {
        self.content.push_str(&include(tree.root()));
        self.tree.append(tree).unwrap();
        self
    }
    pub fn content(mut self, text: &str) -> Self {
        self.content.push_str(&text);
        self
    }
    pub fn build(mut self, name: String, prefix: String) -> SourceInfo {
        self.tree.insert(self.tree.root().to_path_buf(), self.content).unwrap();
        SourceInfo::new(name, prefix, self.tree)
    }
}

pub fn include<P: AsRef<Path>>(path: P) -> String {
    format!("#include <{}>\n", path.as_ref())
}
pub fn include_template<P: AsRef<Path>>(path: P, map: &BTreeMap<String, String>) -> String {
    let mut lines = Vec::new();
    for (key, value) in map.iter() {
        lines.push(format!("#define ${} {}\n", key, value));
    }
    lines.push(format!("#include <{}>\n", path.as_ref()));
    for (key, _) in map.iter() {
        lines.push(format!("#undef ${}\n", key));
    }
    lines.join("")
}

#[macro_export]
macro_rules! include_template {
    ($path:expr, $($name:literal: $value:expr),* $(,)?) => {
        include_template(
            $path,
            &vec![$((String::from($name), String::from($value))),*].into_iter().collect(),
        )
    };
}
