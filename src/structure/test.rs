use std::io;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;
use super::node::Node;
use std::fs::{self, File};
use super::ptree::ProjectTree;
use super::operations::compute_hash;
use super::serialization::{save, load};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;
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
            compute_hash(&file_path).unwrap().as_str()
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

    #[test]
    fn test_save_and_load() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?; // Create a temporary file
        let file_path = temp_file.path().to_path_buf();

        // Create a test ProjectTree with a directory and a file
        let mut tree = ProjectTree {
            root: Node::Directory { children: HashMap::new() },
            base_path: PathBuf::from("/test"),
        };

        // Add a sample file to the tree
        if let Node::Directory { children } = &mut tree.root {
            children.insert(
                "file.txt".to_string(),
                Node::File { hash: "dummyhash".to_string(), modified: true },
            );
        }

        // Save the tree
        save(&tree, Option::from(file_path.clone()))?;

        // Load the tree back
        let loaded_tree = load(Option::from(file_path.clone()))?;

        // Check if the saved and loaded trees are the same
        assert_eq!(
            serde_json::to_string_pretty(&tree)?,
            serde_json::to_string_pretty(&loaded_tree)?
        );

        // Cleanup: Remove the temp file
        fs::remove_file(file_path)?;

        Ok(())
    }

    #[test]
    fn test_modification() {
        // Setup
        let base_path = Path::new("./test_project");
        fs::create_dir_all(base_path).expect("Failed to create base directory");

        let file_path = base_path.join("file.txt");

        // Create ProjectTree
        let mut project_tree = ProjectTree::new(base_path).expect("Failed to create project tree");

        // Step 1: Add the file with initial content
        create_file(&file_path, "Initial content");

        // Add file to project tree
        project_tree.add(&file_path).expect("Failed to add file to tree");

        // Step 2: Store the initial hash (borrow project_tree immutably)
        let initial_hash = {
            project_tree.get_file_hash(&Path::new("file.txt"))
                .expect("Failed to get file hash")
        };

        // Step 3: Modify the file
        create_file(&file_path, "Modified content");

        // Step 4: Add the file again to update the tree with the new content (mutably borrow project_tree)
        project_tree.add(&file_path).expect("Failed to add modified file to tree");

        // Step 5: Get the new hash (borrow project_tree immutably again)
        let new_hash = {
            project_tree.get_file_hash(&Path::new("file.txt"))
                .expect("Failed to get modified file hash")
        };

        // Step 6: Ensure the hash has changed (indicating modification)
        assert_ne!(initial_hash, new_hash, "File should have been marked as modified");

        // Step 7: Check if the file is marked as modified
        let modified_files = project_tree.get_modified_files();
        assert_eq!(modified_files.len(), 1, "There should be one modified file");
        assert_eq!(modified_files[0], PathBuf::from("file.txt"), "The modified file path is incorrect");

        // Cleanup
        fs::remove_dir_all(base_path).expect("Failed to clean up test directory");
    }
}