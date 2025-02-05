use uuid::Uuid;
use std::path::Path;
use std::collections::HashMap;
use super::versioning::Version;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::structure::serialization::{load, save};
use crate::systems::commits::functions::{create_commit_files, save_commit};
use crate::systems::filters::filter::{filter_paths, load_file_filter};
use crate::systems::init::get_current_branch;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub id: String,
    pub timestamp: u64,
    pub author: String,
    pub branch: String,
    pub message: String,
    pub files: Vec<String>,
    pub versions: HashMap<String, String>, // Maps file paths to version IDs
}

impl Commit {
    pub fn new(message: &str, author: &str) -> std::io::Result<Option<Commit>> {
        let mut tree = load(None)?;
        let modified_files = filter_paths(tree.get_modified_files(), &load_file_filter(".filter")?);
        save(&tree, None)?;
        let mut versions_map: HashMap<String, _> = HashMap::new();
        let mut has_actual_changes = false;
        let branch = get_current_branch()?;  // Extract branch name first

        for file_path in &modified_files {
            let file_stem = Path::new(file_path)
                .display()
                .to_string()
                .replace("/", "_") // Works for Unix but not Windows
                .replace("\\", "_"); // Ensures Windows compatibility

            let version_path = format!(".grit/versions/{}/{}.json", branch, file_stem);
            let mut version = match Version::load(&version_path) {
                Ok(v) => v,
                Err(_) => {
                    Version::create(&file_path.to_string_lossy(), &version_path)?;
                    Version::load(&version_path)?
                }
            };

            let version_id = version.add_version(&file_path.to_string_lossy(), &version_path)?;

            if !version_id.is_empty() {
                versions_map.insert(file_path.to_string_lossy().to_string(), version_id);
                has_actual_changes = true;
            } else {
                eprintln!("Warning: version ID is empty for {:?}", file_path);
            }
        }

        let commit = Commit {
            id: Uuid::new_v4().to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_secs(),
            author: author.to_string(),
            branch,
            message: message.to_string(),
            files: modified_files.iter().map(|f| f.to_string_lossy().to_string()).collect(),
            versions: versions_map,
        };
        create_commit_files()?;
        save_commit(&commit)?;

        // Avoid creating a commit if no actual changes were recorded
        if !has_actual_changes {
            return Ok(None);
        }

        Ok(Some(commit))
    }
}
