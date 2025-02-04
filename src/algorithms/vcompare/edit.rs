use serde::{Serialize, Deserialize};

/// This enum demonstrates the actions that can be taken during our comparison.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Edit {
    /// Represents a deletion at a specific index.
    Delete(usize),
    /// Represents an insertion of a string at a specific index.
    Insert(usize, String),
    /// Represents a replacement of a string at a specific index.
    Replace(usize, String),
}