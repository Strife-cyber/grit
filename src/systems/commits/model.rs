use serde::{Deserialize, Serialize};

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