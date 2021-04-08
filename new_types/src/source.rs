use std::collections::HashMap;

pub struct SourceInfo {
    pub name: String,
    pub prefix: String,
    pub source_tree: Option<SourceTree>,
}

impl SourceInfo {
    pub fn new(name: String, prefix: String) -> Self {
        Self {
            name,
            prefix,
            source_tree: None,
        }
    }
}

pub struct SourceTree {
    pub root: String,
    pub tree: HashMap<String, String>,
}
