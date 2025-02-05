use std::{io, fs};
use std::io::Read;
use std::fs::File;
use std::path::Path;
use super::commit::Commit;
use std::collections::HashMap;

const COMMITS_FILE: &str = ".grit/commits.json";
pub const HEAD_FILE: &str = ".grit/HEAD";

pub fn create_commit_files() -> io::Result<()> {
    // Ensure HEAD file exists before writing
    if !Path::new(HEAD_FILE).exists() || !Path::new(COMMITS_FILE).exists() {
        if let Some(parent) = Path::new(COMMITS_FILE).parent() {
            fs::create_dir_all(parent)?;
        }
        if let Some(parent) = Path::new(HEAD_FILE).parent() {
            fs::create_dir_all(parent)?;
        }

        File::create(COMMITS_FILE)?;
        File::create(HEAD_FILE)?; // Create HEAD file if it doesn’t exist
    }

    Ok(())
}

/// Save a new commit and update HEAD
pub fn save_commit(commit: &Commit) -> io::Result<()> {
    let mut commits = load_all_commits()?; // Avoid unwrap()

    // Save the new commit
    commits.insert(commit.id.clone(), commit.clone());
    let json = serde_json::to_string_pretty(&commits)?;
    fs::write(COMMITS_FILE, json)?;
    fs::write(HEAD_FILE, &commit.id.trim())?;

    Ok(())
}

/// Get the last commit ID from HEAD
pub fn get_head_commit() -> io::Result<Option<String>> {
    if Path::new(HEAD_FILE).exists() {
        let mut head = String::new();
        File::open(HEAD_FILE)?.read_to_string(&mut head)?;
        return Ok(Some(head.trim().to_string())); // Trim any newlines
    }
    Ok(None)
}

/// Load a commit by ID
pub fn load_commit(commit_id: &str) -> io::Result<Option<Commit>> {
    let commits = load_all_commits()?;
    Ok(commits.get(commit_id).cloned())
}

/// Load all commits
pub fn load_all_commits() -> io::Result<HashMap<String, Commit>> {
    let json = fs::read_to_string(COMMITS_FILE)?;
    let commits: HashMap<String, Commit> = serde_json::from_str(&json).unwrap_or_else(|_| HashMap::new()); // Handle invalid JSON
    Ok(commits)
}

/// Reads a file as UTF-8 or falls back to best-effort conversion
pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut raw_content = Vec::new();
    File::open(file_path)?.read_to_end(&mut raw_content)?;

    match std::str::from_utf8(&raw_content) {
        Ok(text) => Ok(text.to_string()), // Return valid UTF-8 string
        Err(_) => {
            println!("Warning: File is not UTF-8 encoded, converting with loss."); // More descriptive message
            Ok(String::from_utf8_lossy(&raw_content).to_string()) // Convert with lossy UTF-8
        }
    }
}
