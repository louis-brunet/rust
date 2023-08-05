use std::{collections::HashMap, fmt::Display, str::FromStr};

use super::parser::{self, FileTreeParseError};

#[derive(Debug)]
pub enum FsNode {
    File(u32),
    Directory(HashMap<String, FsNode>),
}

impl FsNode {
    pub fn to_tree_string(&self, name: &str, indent: u8) -> String {
        format!(
            "{:width$}- {} {}", 
            "", 
            name,
            match self {
                FsNode::File(size) => format!("(file, size={})", size),
                FsNode::Directory(children) => format!(
                    "(dir){}",
                    children.iter()
                        .map(|(name, node)| node.to_tree_string(name, indent + 1))
                        .fold(String::new(), |acc, tree_str| format!("{}\n{}", acc, tree_str))
                ),
            }, 
            width = indent as usize * 2,
        )
    }

    fn find_child_directories<'a, P>(&'a self, predicate: &P, dirs: &mut Vec<(&'a str, u32)>) -> u32 
    where 
        P: Fn(u32) -> bool
    {
        let mut self_size = 0;

        match self {
            FsNode::File(size) => self_size = *size,
            FsNode::Directory(children) => {
                for (name, node) in children.iter() {
                    let child_size = node.find_child_directories(predicate, dirs);
                    self_size += child_size;

                    if let FsNode::Directory(_) = node {
                        if predicate(child_size) {
                            dirs.push((name, child_size));
                        }
                    }
                }
            },
        }

        return self_size;
    }

    pub fn find_small_child_directories<'a>(&'a self, max_size: u32, small_dirs: &mut Vec<(&'a str, u32)>) -> u32 {
        let is_in_bounds = |size| size <= max_size;
        return self.find_child_directories(&is_in_bounds, small_dirs);
    }

    pub fn find_large_child_directories<'a>(&'a self, min_size: u32, dirs: &mut Vec<(&'a str, u32)>) -> u32 {
        let is_in_bounds = |size| size >= min_size;
        return self.find_child_directories(&is_in_bounds, dirs);
    }
}

#[derive(Debug)]
pub struct FileTree {
    pub root: FsNode,
}

impl FileTree {
    pub fn new() -> FileTree {
        FileTree { root: FsNode::Directory(HashMap::new()) }
    }
}

impl FromStr for FileTree {
    type Err = FileTreeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return parser::parse_file_tree(s);
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.root.to_tree_string("/", 1),
        )
    }
}

