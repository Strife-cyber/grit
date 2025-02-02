use std::fs;
use std::io;
use sha1::{Sha1, Digest};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    File { hash: String },
    Directory { children: HashMap<String, Node> },
}

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
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidInput,
                "Path is outside project directory"
            ))?;

        if abs_path.is_file() {
            let current_hash = self.compute_hash(&abs_path)?;
            self.add_file(relative_path, current_hash)?;
        } else if abs_path.is_dir() {
            self.add_all(relative_path)?;
        }
        Ok(())
    }

    /// Add a single file using relative path
    fn add_file(&mut self, rel_path: &Path, current_hash: String) -> io::Result<()> {
        let components: Vec<&str> = rel_path.iter()
            .filter_map(|c| c.to_str())
            .collect();

        if components.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty path"));
        }

        let (file_component, dir_components) = components.split_last().unwrap();
        let mut current = &mut self.root;

        // Build directory structure
        for component in dir_components {
            current = match current {
                Node::Directory { children } => {
                    children.entry((*component).to_string())
                        .or_insert_with(|| Node::Directory {
                            children: HashMap::new(),
                        })
                }
                _ => return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid path: expected directory",
                )),
            };
        }

        // Insert file
        if let Node::Directory { children } = current {
            let file_name = (*file_component).to_string();

            // Check for existing file
            if let Some(Node::File { hash: existing }) = children.get(&file_name) {
                if existing != &current_hash {
                    println!("Modified: {}", self.base_path.join(rel_path).display());
                }
            } else {
                println!("Added: {}", self.base_path.join(rel_path).display());
            }

            children.insert(file_name, Node::File {
                hash: current_hash,
            });
        }

        Ok(())
    }

    /// Add directory contents recursively
    fn add_all(&mut self, rel_path: &Path) -> io::Result<()> {
        let abs_path = self.base_path.join(rel_path);
        for entry in fs::read_dir(abs_path)? {
            let entry = entry?;
            let entry_path = entry.path();
            self.add(&entry_path)?;
        }
        Ok(())
    }

    /// Compute SHA-1 hash of file contents
    pub(crate) fn compute_hash(&self, path: &Path) -> io::Result<String> {
        let content = fs::read(path)?;
        let mut hasher = Sha1::new();
        hasher.update(&content);
        Ok(hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect())
    }

    /// Get file hash by relative path
    pub fn get_file_hash(&self, rel_path: &Path) -> Option<&String> {
        let components: Vec<&str> = rel_path.iter()
            .filter_map(|c| c.to_str())
            .collect();

        if components.is_empty() {
            return None;
        }

        let (file_component, dir_components) = components.split_last().unwrap();
        let mut current = &self.root;

        for component in dir_components {
            current = match current {
                Node::Directory { children } => children.get(*component)?,
                _ => return None,
            };
        }

        match current {
            Node::Directory { children } => {
                if let Node::File { hash } = children.get(*file_component)? {
                    Some(hash)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Check if a path exists in the tree
    pub fn exists(&self, rel_path: &Path) -> bool {
        self.get_file_hash(rel_path).is_some()
    }

    /// List all files in the tree with relative paths
    pub fn list_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        self.traverse(&self.root, PathBuf::new(), &mut files);
        files
    }

    fn traverse(&self, node: &Node, current_path: PathBuf, files: &mut Vec<PathBuf>) {
        match node {
            Node::File { .. } => {
                files.push(current_path);
            }
            Node::Directory { children } => {
                for (name, node) in children {
                    let path = current_path.join(name);
                    self.traverse(node, path, files);
                }
            }
        }
    }

    pub fn save(&self, file_path: &Path) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load(file_path: &Path) -> io::Result<Self> {
        let mut file = fs::File::open(file_path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        let tree: ProjectTree = serde_json::from_str(&json)?;
        Ok(tree)
    }
}