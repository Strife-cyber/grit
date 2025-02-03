use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    File { hash: String, modified: bool },
    Directory { children: HashMap<String, Node> },
}