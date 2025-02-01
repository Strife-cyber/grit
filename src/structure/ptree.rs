use std::fs;
use std::io;
use sha1::{Sha1, Digest};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Node {
    File { hash: String, path: PathBuf },
    Directory { children: HashMap<String, Node>, path: PathBuf },
}

#[derive(Debug)]
pub struct ProjectTree {
    root: Node,
}

impl ProjectTree {
    /// Create a new empty project tree.
    fn new() -> Self {
        ProjectTree {
            root: Node::Directory {
                children: HashMap::new(),
                path: PathBuf::from("."),
            },
        }
    }

    /// Add a file or directory to the tree.
    fn add(&mut self, path: &Path) -> io::Result<()> {
        if path.is_file() {
            let current_hash = self.compute_hash(path)?;
            self.add_file(path, current_hash)?;
        } else if path.is_dir() {
            self.add_all(path)?;
        }
        Ok(())
    }

    /// Add a single file to the tree.
    fn add_file(&mut self, path: &Path, current_hash: String) -> io::Result<()> {
        let mut current = &mut self.root;
        let components: Vec<&str> = path.iter().map(|c| c.to_str().unwrap()).collect();

        for (i, component) in components.iter().enumerate() {
            if let Node::Directory { children, path: dir_path } = current {
                if i == components.len() - 1 {
                    // Last component is the file to add
                    if let Some(Node::File { hash: stored_hash, .. }) = children.get(component) {
                        if stored_hash != &current_hash {
                            println!("File modified: {}", path.display());
                        }
                    } else {
                        println!("New file added: {}", path.display());
                    }

                    // Update the file in the tree
                    children.insert(component.to_string(), Node::File {
                        hash: current_hash.clone(),
                        path: path.to_path_buf(),
                    });
                } else {
                    // Traverse into the directory
                    current = children.entry(component.to_string()).or_insert_with(|| Node::Directory {
                        children: HashMap::new(),
                        path: dir_path.join(component),
                    });
                }
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid path: expected a directory",
                ));
            }
        }
        Ok(())
    }

    /// Add all files in a directory recursively.
    fn add_all(&mut self, path: &Path) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            self.add(&entry_path)?;
        }
        Ok(())
    }

    /// Compute the hash of a file using SHA-1.
    fn compute_hash(&self, path: &Path) -> io::Result<String> {
        let content = fs::read(path)?; // Read the file content
        let mut hasher = Sha1::new();
        hasher.update(&content);
        let result = hasher.finalize(); // SHA-1 digest

        // Convert hash bytes to a hex string
        Ok(result.iter().map(|b| format!("{:02x}", b)).collect())
    }
}
