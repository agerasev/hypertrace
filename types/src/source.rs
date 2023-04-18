use std::{
    collections::{
        btree_map::{BTreeMap, Entry},
        HashSet,
    },
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct ContentMismatchError {
    pub path: PathBuf,
    pub src: String,
    pub dst: String,
}

impl From<ContentMismatchError> for String {
    fn from(err: ContentMismatchError) -> String {
        format!(
            "Files {:?} has the same path but different content",
            err.path
        )
    }
}

#[derive(Clone, Default)]
pub struct SourceTree {
    root: PathBuf,
    files: BTreeMap<PathBuf, String>,
}

impl SourceTree {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self {
            root: root.into(),
            files: BTreeMap::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = root;
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Path, &String)> {
        self.files.iter().map(|(x, y)| (x.as_path(), y))
    }

    pub fn insert(&mut self, name: PathBuf, content: String) -> Result<(), ContentMismatchError> {
        match self.files.entry(name) {
            Entry::Vacant(ve) => {
                ve.insert(content);
                Ok(())
            }
            Entry::Occupied(oe) => {
                let src = oe.get();
                if src == &content {
                    Ok(())
                } else {
                    Err(ContentMismatchError {
                        path: oe.key().clone(),
                        src: src.clone(),
                        dst: content,
                    })
                }
            }
        }
    }

    pub fn append(&mut self, mut other: SourceTree) -> Result<(), ContentMismatchError> {
        let self_keys = self.files.keys().collect::<HashSet<_>>();
        let other_keys = other.files.keys().collect::<HashSet<_>>();
        let collision = self_keys.intersection(&other_keys).copied();
        for path in collision {
            let (src, dst) = (
                self.files.get(path).unwrap(),
                other.files.get(path).unwrap(),
            );
            if src != dst {
                return Err(ContentMismatchError {
                    path: path.clone(),
                    src: src.clone(),
                    dst: dst.clone(),
                });
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
        self.content.push_str(text);
        self
    }
    pub fn build(mut self) -> SourceTree {
        self.tree
            .insert(self.tree.root().to_path_buf(), self.content)
            .unwrap();
        self.tree
    }
}

pub fn include<P: AsRef<Path>>(path: P) -> String {
    format!("#include <{}>\n", path.as_ref().to_str().unwrap())
}
pub fn include_template<P: AsRef<Path>>(path: P, map: &BTreeMap<String, String>) -> String {
    let mut lines = Vec::new();
    for (key, value) in map.iter() {
        lines.push(format!("#define ${} {}\n", key, value));
    }
    lines.push(format!("#include <{}>\n", path.as_ref().to_str().unwrap()));
    for (key, _) in map.iter() {
        lines.push(format!("#undef ${}\n", key));
    }
    lines.join("")
}
pub fn type_alias(dst: &str, src: &str) -> String {
    format!("typedef {} {};\n", src, dst)
}

#[macro_export]
macro_rules! _include_template_unwind {
    ($map:ident, $name:literal => $value:expr) => {
        assert!($map.insert(String::from($name), $value).is_none());
    };
    ($map:ident, ($type_name:literal, $prefix:literal) => $value:expr) => {
        assert!($map.insert(String::from($type_name), $value.0).is_none());
        assert!($map.insert(String::from($prefix), $value.1).is_none());
    };
}
#[macro_export]
macro_rules! include_template {
    ($path:expr, $($key:tt => $value:expr),* $(,)?) => {{
        let mut args = std::collections::BTreeMap::new();
        $(
            $crate::_include_template_unwind!(args, $key => $value);
        )*
        $crate::source::include_template($path, &args)
    }};
}
