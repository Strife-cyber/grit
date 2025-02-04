use uuid::Uuid;
use std::path::Path;
use std::collections::HashMap;
use super::versioning::Version;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::structure::serialization::{load, save};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub id: String,
    pub timestamp: u64,
    pub author: String,
    pub message: String,
    pub files: Vec<String>,
    pub versions: HashMap<String, String>, // Maps file paths to version IDs
}

impl Commit {
    pub fn new(message: &str, author: &str) -> std::io::Result<Option<Commit>> {
        let mut tree = load(None)?;
        let modified_files = tree.get_modified_files();
        save(&tree, None)?;
        let mut versions_map: HashMap<String, _> = HashMap::new();
        let mut has_actual_changes = false;

        for file_path in &modified_files {
            let file_stem = Path::new(file_path)
                .with_extension("")
                .to_string_lossy()
                .replace("/", "_");
            let version_path = format!(".grit/versions/{}.json", file_stem);
            let mut version = match Version::load(&version_path) {
                Ok(v) =>v,
                Err(_) => {
                    Version::create(file_path.to_str().unwrap(), &version_path)?;
                    Version::load(&version_path)?
                }
            };

            let version_id = version.add_version(file_path.to_str().unwrap(), &version_path)?;

            if !version_id.is_empty() {
                versions_map.insert(file_path.clone().to_str().unwrap().to_string(), version_id);
                has_actual_changes = true;
            }
        }

        // Avoid creating a commit if no actual changes were recorded
        if !has_actual_changes {
            return Ok(None);
        }

        Ok(Some(Commit {
            id: Uuid::new_v4().to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            author: author.to_string(),
            message: message.to_string(),
            files: modified_files.iter().map(|f| f.to_str().unwrap().to_string()).collect(),
            versions: versions_map,
        }))
    }
}
