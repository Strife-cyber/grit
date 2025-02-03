use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::structure::serialization::load;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileVersion {
    pub path: String,
    pub object_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub author: String,
    pub timestamp: u64,
    pub message: String,
    pub parent: Option<String>,
    pub tracked_files: Vec<FileVersion>
}

impl Commit {
    /// Generate a new commit
    pub fn new(
        author: String, message: String,
        parent: Option<String>, tracked_files: Vec<FileVersion>
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let id = Uuid::new_v4().to_string();

        Commit {
            id, author, message, parent, tracked_files, timestamp
        }
    }

    pub fn construct_commit(author: String, message: String) {
        let tree = load(None).unwrap();
        let modified = tree.get_modified_files();
    }
}