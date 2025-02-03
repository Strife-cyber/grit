use super::node::Node;
use std::path::PathBuf;

pub fn traverse(node: &Node, current_path: PathBuf, files: &mut Vec<PathBuf>) {
    match node {
        Node::File { .. } => files.push(current_path),
        Node::Directory { children } => {
            for (name, node) in children {
                let path = current_path.join(name);
                traverse(node, path, files);
            }
        }
    }
}


#[allow(dead_code)]
pub fn traverse_modified(node: &Node, current_path: PathBuf, modified_files: &mut Vec<PathBuf>) {
    match node {
        Node::File { modified, .. } if *modified => modified_files.push(current_path),
        Node::Directory { children } => {
            for (name, node) in children {
                let path = current_path.join(name);
                traverse_modified(node, path, modified_files);
            }
        }
        _ => {}
    }
}