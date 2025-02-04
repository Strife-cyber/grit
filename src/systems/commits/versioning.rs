use uuid::Uuid;
use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};
use chardetng::EncodingDetector;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::algorithms::vcompare::edit::Edit;
use crate::algorithms::vcompare::compv::compare;
use crate::algorithms::vcompare::utils::split_lines;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub original: String,
    pub versions: Vec<VersionData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionData {
    pub version_id: String,
    pub timestamp: u64,
    pub changes: Vec<Edit>,
}

impl Version {
    /// Reads a file and converts it to UTF-8 if necessary
    fn read_file_as_utf8(file_path: &str) -> std::io::Result<String> {
        let mut raw_content = Vec::new();
        File::open(file_path)?.read_to_end(&mut raw_content)?;

        // Detect encoding
        let mut detector = EncodingDetector::new();
        detector.feed(&raw_content, true);
        let encoding = detector.guess(None, true);

        // Decode with detected encoding
        let (content, _, had_errors) = encoding.decode(&raw_content);
        if had_errors {
            eprintln!("Warning: Some characters could not be decoded properly.");
        }

        Ok(content.into_owned())
    }

    /// Creates a new version-tracked file
    pub fn create(file_path: &str, json_path: &str) -> std::io::Result<()> {
        let content = Self::read_file_as_utf8(file_path)?;

        let version_data = Version {
            original: content.clone(),
            versions: Vec::new(),
        };

        let json = serde_json::to_string_pretty(&version_data)?;

        // Ensure the directory exists before writing
        if let Some(parent) = Path::new(json_path).parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(json_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Load an existing tracked file
    pub fn load(json_path: &str) -> std::io::Result<Version> {
        let json_content = Self::read_file_as_utf8(json_path)?;
        let version_data: Version = serde_json::from_str(&json_content)?;
        Ok(version_data)
    }

    /// Adds a new version by computing differences
    pub fn add_version(&mut self, file_path: &str, json_path: &str) -> std::io::Result<String> {
        let new_content = Self::read_file_as_utf8(file_path)?.trim_end().to_string();
        let last_content = self.reconstruct_latest().trim_end().to_string();

        let changes = compare(&last_content, &new_content);

        // Avoid adding an unnecessary version if no real changes exist
        if changes.is_empty() {
            return Ok("".to_string());
        }

        let id = Uuid::new_v4().to_string();

        let new_version = VersionData {
            version_id: id.clone(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            changes,
        };

        self.versions.push(new_version);

        let json = serde_json::to_string_pretty(&self)?;
        fs::write(json_path, json)?;

        Ok(id)
    }

    /// Reconstructs the latest version from stored data
    pub fn reconstruct_latest(&self) -> String {
        let mut content = self.original.clone();
        for version in &self.versions {
            content = apply_changes(&content, &version.changes);
        }
        content
    }

    /// Retrieves a specific version by reconstructing it
    pub fn reconstruct_version(&self, version_id: &str) -> Option<String> {
        let mut content = self.original.clone();
        for version in &self.versions {
            content = apply_changes(&content, &version.changes);
            if version.version_id == version_id {
                return Some(content);
            }
        }
        None
    }
}

/// Applies a list of changes to a string and returns the modified result
fn apply_changes(content: &str, changes: &[Edit]) -> String {
    let mut lines: Vec<String> = split_lines(content);

    for change in changes {
        match change {
            Edit::Insert(index, text) => {
                if *index < lines.len() {
                    lines.insert(*index, text.clone());
                }
            }
            Edit::Delete(index) => {
                if *index < lines.len() {
                    lines.remove(*index);
                }
            }
            Edit::Replace(index, text) => {
                if *index < lines.len() {
                    lines[*index] = text.clone();
                }
            }
        }
    }

    lines.join("\n")
}
