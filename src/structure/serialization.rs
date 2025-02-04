use serde_json;
use std::path::PathBuf;
use super::ptree::ProjectTree;
use std::{fs::{self, File}, io};

const DEFAULT_GRIT_TREE_FILE: &str = ".grit/tree.json";

pub fn save(tree: &ProjectTree, file_path: Option<PathBuf>) -> io::Result<()> {
    let path: PathBuf = file_path.unwrap_or_else(|| PathBuf::from(DEFAULT_GRIT_TREE_FILE));

    // Ensure the .grit directory exists before writing
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(tree)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load(file_path: Option<PathBuf>) -> io::Result<ProjectTree> {
    let path: PathBuf = file_path.unwrap_or_else(|| PathBuf::from(DEFAULT_GRIT_TREE_FILE));

    // Ensure the .grit directory exists before writing
    if !path.exists() {
        File::create(&path)?;
    }

    let json = fs::read_to_string(path.clone())?;

    // Handle empty file case safely
    let tree: ProjectTree = if json == "{}" {
        ProjectTree::new(
            path
        )? // Assuming ProjectTree implements Default
    } else {
        serde_json::from_str(&json)?
    };
    Ok(tree)
}
