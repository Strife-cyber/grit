use std::io;
use super::node::Node;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use super::operations::{
    compute_hash, add_all,
    add_file, get_node
};
use super::transversal::{traverse, traverse_modified};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTree {
    pub(crate) root: Node,
    pub(crate) base_path: PathBuf,
}

impl ProjectTree {
    /// Create a new project tree rooted at the given path
    pub fn new(base_path: impl Into<PathBuf>) -> io::Result<Self> {
        let base_path = base_path.into().canonicalize()?;
        Ok(ProjectTree {
            root: Node::Directory { children: HashMap::new() },
            base_path,
        })
    }

    /// Add a file or directory to the tree
    pub fn add(&mut self, path: &Path) -> io::Result<()> {
        let abs_path = path.canonicalize()?;
        let relative_path = abs_path.strip_prefix(&self.base_path)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Path is outside project directory"))?;

        if abs_path.is_file() {
            let current_hash = compute_hash(&abs_path)?;
            add_file(self, relative_path, current_hash)?;
        } else if abs_path.is_dir() {
            add_all(self, relative_path)?;
        }
        Ok(())
    }

    /// Get file hash by relative path, ignoring modification status
    /// Get file hash by relative path, allowing modification of the project tree
    pub fn get_file_hash(&mut self, rel_path: &Path) -> Option<String> {
        // Borrow the node mutably, but avoid returning a reference that outlives the borrow
        get_node(&mut self.root, rel_path).and_then(|node| {
            if let Node::File { hash, .. } = node {
                Some(hash.clone()) // Clone the hash to return it safely
            } else {
                None
            }
        })
    }

    /// Check if a path exists in the tree
    pub fn exists(&self, rel_path: &Path) -> bool {
        get_node(&self.root, rel_path).is_some()
    }

    /// List all files in the tree
    pub fn list_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        traverse(&self.root, PathBuf::new(), &mut files);
        files
    }

    /// Get a list of modified files

    #[allow(dead_code)]
    pub fn get_modified_files(&self) -> Vec<PathBuf> {
        let mut modified_files = Vec::new();
        traverse_modified(&self.root, PathBuf::new(), &mut modified_files);
        modified_files
    }
}