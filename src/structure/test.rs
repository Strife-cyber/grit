use std::io::Write;
use std::path::Path;
use tempfile::TempDir;
use std::fs::{self, File};

use super::ptree::{Node, ProjectTree};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    fn create_file(path: &Path, content: &str) {
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(content.as_bytes()).expect("Failed to write to file");
    }

    #[test]
    fn test_create_new_tree() {
        let tmp_dir = TempDir::new().unwrap();
        let tree = ProjectTree::new(tmp_dir.path()).unwrap();

        match tree.root {
            Node::Directory { ref children } => {
                assert!(children.is_empty(), "Tree should be empty on initialization");
            }
            _ => panic!("Root should be a directory"),
        }
    }

    #[test]
    fn test_add_single_file() {
        let tmp_dir = TempDir::new().unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        create_file(&file_path, "Hello, World!");

        let mut tree = ProjectTree::new(tmp_dir.path()).unwrap();
        tree.add(&file_path).unwrap();

        assert!(tree.exists(Path::new("test.txt")), "File not found in tree");
        assert_eq!(
            tree.get_file_hash(Path::new("test.txt")).unwrap(),
            tree.compute_hash(&file_path).unwrap().as_str()
        );
    }

    #[test]
    fn test_add_multiple_files_in_directory() {
        let tmp_dir = TempDir::new().unwrap();
        let file1 = tmp_dir.path().join("file1.txt");
        let file2 = tmp_dir.path().join("file2.txt");
        create_file(&file1, "Content 1");
        create_file(&file2, "Content 2");

        let mut tree = ProjectTree::new(tmp_dir.path()).unwrap();
        tree.add(tmp_dir.path()).unwrap();

        let files = tree.list_files();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&PathBuf::from("file1.txt")));
        assert!(files.contains(&PathBuf::from("file2.txt")));
    }

    #[test]
    fn test_add_nested_directories_with_files() {
        let tmp_dir = TempDir::new().unwrap();
        let subdir = tmp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file1 = subdir.join("file1.txt");
        let file2 = tmp_dir.path().join("file2.txt");
        create_file(&file1, "Nested file content");
        create_file(&file2, "Root file content");

        let mut tree = ProjectTree::new(tmp_dir.path()).unwrap();
        tree.add(tmp_dir.path()).unwrap();

        assert!(tree.exists(Path::new("file2.txt")));
        assert!(tree.exists(Path::new("subdir/file1.txt")));
    }

    #[test]
    fn test_file_modification_detection() {
        let tmp_dir = TempDir::new().unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        create_file(&file_path, "Initial Content");

        let mut tree = ProjectTree::new(tmp_dir.path()).unwrap();
        tree.add(&file_path).unwrap();

        // Store the original hash in a separate variable
        let original_hash = tree.get_file_hash(Path::new("test.txt"))
            .unwrap()
            .clone(); // Clone to avoid holding reference

        // Modify file
        create_file(&file_path, "Modified Content");
        tree.add(&file_path).unwrap();

        // Get new hash
        let new_hash = tree.get_file_hash(Path::new("test.txt")).unwrap();

        // Compare hashes
        assert_ne!(original_hash, new_hash.to_string());
    }

    #[test]
    fn test_handling_non_existent_file() {
        let tmp_dir = TempDir::new().unwrap();
        let mut tree = ProjectTree::new(tmp_dir.path()).unwrap();
        let bad_path = tmp_dir.path().join("ghost.txt");

        let result = tree.add(&bad_path);
        assert!(result.is_err(), "Should error on non-existent file");
    }

    #[test]
    fn test_empty_directory_handling() {
        let tmp_dir = TempDir::new().unwrap();
        let mut tree = ProjectTree::new(tmp_dir.path()).unwrap();

        tree.add(tmp_dir.path()).unwrap();
        assert_eq!(tree.list_files().len(), 0);
    }
}