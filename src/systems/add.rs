use std::io;
use std::fs;
use std::env;
use crate::structure::ptree::ProjectTree;
use crate::systems::init::find_grit_root;
use crate::structure::serialization::save;

/// Adds files to the Grit repository. If a path is provided, it adds the file if it exists.
/// Otherwise, it adds all files in the current directory except `.grit/`.
pub fn add(path: Option<&str>) -> io::Result<()> {
    // Get the current directory
    let current_dir = env::current_dir()?;

    // Locate the Grit repository root
    let root = match find_grit_root(&current_dir) {
        Some(root) => root.canonicalize()?, // Normalize path
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "No .grit repository found in this directory or any parent directory.")),
    };

    // Ensure we are inside a Grit repository before adding files
    if !root.join(".grit").exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No .grit directory found. Please initialize a Grit repository first."));
    }

    let mut files_to_add = Vec::new();

    if let Some(p) = path {
        // Handle adding a specific file/directory
        let abs_path = root.join(p); // Ensure path is relative to Grit root
        if abs_path.exists() {
            files_to_add.push(abs_path);
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Specified file does not exist"));
        }
    } else {
        // Handle adding all files except `.grit/`
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() && entry_path.file_name().map_or(false, |name| name == ".grit") {
                continue; // Skip .grit directory
            }

            files_to_add.push(entry_path);
        }
    }

    // Initialize the project tree
    let mut tree = ProjectTree::new(root)?;

    // Add each file to the tree
    for file in files_to_add {
        tree.add(&file)?;
    }

    save(&tree, None)?;

    Ok(())
}
