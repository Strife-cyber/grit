use std::{fs, io};
use super::node::Node;
use sha1::{Sha1, Digest};
use super::ptree::ProjectTree;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use hex::encode;

pub fn compute_hash(path: &Path) -> io::Result<String> {
    let content = fs::read(path)?;
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(encode(&result))
}

pub fn add_file(tree: &mut ProjectTree, rel_path: &Path, current_hash: String) -> io::Result<()> {
    let (file_name, dir_components) = split_path(rel_path)?;
    let parent_node = get_or_create_parent_node(&mut tree.root, dir_components)?;

    if let Node::Directory { children } = parent_node {
        update_or_insert_file(children, file_name, current_hash, rel_path, &tree.base_path);
    }

    Ok(())
}

pub fn add_all(tree: &mut ProjectTree, rel_path: &Path) -> io::Result<()> {
    let abs_path = tree.base_path.join(rel_path);
    for entry in fs::read_dir(abs_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        tree.add(&entry_path)?;
    }
    Ok(())
}

pub fn get_node<'a>(root: &'a Node, rel_path: &Path) -> Option<&'a Node> {
    let components: Vec<&str> = rel_path.iter().filter_map(|c| c.to_str()).collect();
    let (file_name, dir_components) = components.split_last()?;
    let mut current = root;

    for component in dir_components {
        current = match current {
            Node::Directory { children } => children.get(*component)?,
            _ => return None,
        };
    }

    match current {
        Node::Directory { children } => children.get(*file_name),
        _ => None,
    }
}


fn split_path(rel_path: &Path) -> io::Result<(&str, Vec<&str>)> {
    let components: Vec<&str> = rel_path.iter().filter_map(|c| c.to_str()).collect();
    let (file_name, dir_components) = components.split_last()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Empty path"))?;
    Ok((file_name, dir_components.to_vec()))
}

fn get_or_create_parent_node<'a>(root: &'a mut Node, dir_components: Vec<&str>) -> io::Result<&'a mut Node> {
    let mut current = root;

    for component in dir_components {
        // Use a separate scope to avoid multiple mutable borrows
        if let Node::Directory { children } = current {
            current = children
                .entry(component.to_string())
                .or_insert_with(|| Node::Directory {
                    children: HashMap::new(),
                });
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path: expected directory"));
        }
    }

    Ok(current)
}

fn update_or_insert_file(children: &mut HashMap<String, Node>, file_name: &str, current_hash: String, rel_path: &Path, base_path: &PathBuf) {
    match children.get_mut(file_name) {
        Some(Node::File { hash, modified }) => {
            if *hash != current_hash {
                println!("Modified: {}", base_path.join(rel_path).display());
                *hash = current_hash;
                *modified = true;
            }
        }
        _ => {
            println!("Added: {}", base_path.join(rel_path).display());
            children.insert(file_name.to_string(), Node::File {
                hash: current_hash,
                modified: true, // New files are considered as "modified"
            });
        }
    }
}
