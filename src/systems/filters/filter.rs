use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFilter {
    pub allowed: Option<Vec<String>>, // If set, only these paths are included
    pub denied: Option<Vec<String>>,  // If set, these paths are removed
}

/// Loads the file filter from a `.filter` file (TOML/JSON)
pub fn load_file_filter(file_path: &str) -> io::Result<FileFilter> {
    if !Path::new(file_path).exists() {  // ✅ Corrected
        return Ok(FileFilter { allowed: None, denied: None });
    }

    let content = fs::read_to_string(file_path)?;
    let filter: FileFilter = toml::from_str(&content)
        .or_else(|_| serde_json::from_str(&content)) // Try JSON if TOML fails
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?; // Convert to io::Error

    Ok(filter)
}

/// Filters a list of paths based on the `allowed` and `denied` rules
pub fn filter_paths(paths: Vec<PathBuf>, filter: &FileFilter) -> Vec<PathBuf> {
    let allowed_set: Option<Vec<PathBuf>> = filter.allowed.as_ref().map(|v| v.iter().map(PathBuf::from).collect());
    let denied_set: Option<Vec<PathBuf>> = filter.denied.as_ref().map(|v| v.iter().map(PathBuf::from).collect());

    paths.into_iter()
        .filter(|path| {
            let is_allowed = allowed_set.as_ref().map_or(true, |allowed| {
                allowed.iter().any(|allowed_path| path.starts_with(allowed_path))
            });

            let is_denied = denied_set.as_ref().map_or(false, |denied| {
                denied.iter().any(|denied_path| path.starts_with(denied_path))
            });

            is_allowed && !is_denied
        })
        .collect()
}