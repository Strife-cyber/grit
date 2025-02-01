use super::ptree::{Proj}

#[cfg(test)]
mod tests {

    use std::io::Write;
    use std::path::Path;
    use tempfile::TempDir;
    use std::fs::{self, File};

    /// Helper function to create a file with content
    fn create_file(path: &Path, content: &str) {
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(content.as_bytes()).expect("Failed to write to file");
    }

    #[test]
    fn test_create_new_tree() {
        let tree = ProjectTree::new();
        match tree.root {
            Node::Directory { ref children, .. } => assert!(children.is_empty(), "Tree should be empty on initialization"),
            _ => panic!("Root should be a directory"),
        }
    }

    #[test]
    fn test_add_single_file() {
        let tmp_dir = TempDir::new("test_project").unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        create_file(&file_path, "Hello, World!");

        let mut tree = ProjectTree::new();
        tree.add(&file_path).unwrap();

        match tree.root {
            Node::Directory { ref children, .. } => {
                assert!(children.contains_key("test.txt"), "File should be added to the tree");
            }
            _ => panic!("Root should be a directory"),
        }
    }

    #[test]
    fn test_add_multiple_files_in_directory() {
        let tmp_dir = tempDir::new("test_project").unwrap();
        let file1 = tmp_dir.path().join("file1.txt");
        let file2 = tmp_dir.path().join("file2.txt");

        create_file(&file1, "Content 1");
        create_file(&file2, "Content 2");

        let mut tree = ProjectTree::new();
        tree.add(&tmp_dir.path()).unwrap();

        match tree.root {
            Node::Directory { ref children, .. } => {
                assert!(children.contains_key("file1.txt"), "File1 should be in the tree");
                assert!(children.contains_key("file2.txt"), "File2 should be in the tree");
            }
            _ => panic!("Root should be a directory"),
        }
    }

    #[test]
    fn test_add_nested_directories_with_files() {
        let tmp_dir = TempDir::new("test_project").unwrap();
        let subdir = tmp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let file1 = subdir.join("file1.txt");
        let file2 = tmp_dir.path().join("file2.txt");

        create_file(&file1, "Nested file content");
        create_file(&file2, "Root file content");

        let mut tree = ProjectTree::new();
        tree.add(&tmp_dir.path()).unwrap();

        match &tree.root {
            Node::Directory { ref children, .. } => {
                assert!(children.contains_key("file2.txt"), "Root file should be in the tree");
                assert!(children.contains_key("subdir"), "Subdirectory should be in the tree");

                if let Node::Directory { ref children, .. } = children["subdir"] {
                    assert!(children.contains_key("file1.txt"), "Nested file should be inside subdir");
                } else {
                    panic!("subdir should be a directory");
                }
            }
            _ => panic!("Root should be a directory"),
        }
    }

    #[test]
    fn test_file_modification_detection() {
        let tmp_dir = TempDir::new("test_project").unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        create_file(&file_path, "Initial Content");

        let mut tree = ProjectTree::new();
        tree.add(&file_path).unwrap();

        create_file(&file_path, "Modified Content"); // Modify the file
        tree.add(&file_path).unwrap();

        match tree.root {
            Node::Directory { ref children, .. } => {
                if let Node::File { ref hash, .. } = children["test.txt"] {
                    let new_hash = tree.compute_hash(&file_path).unwrap();
                    assert_eq!(hash, &new_hash, "Hash should update after modification");
                } else {
                    panic!("test.txt should be a file");
                }
            }
            _ => panic!("Root should be a directory"),
        }
    }

    #[test]
    fn test_handling_non_existent_file() {
        let tmp_dir = TempDir::new("test_project").unwrap();
        let non_existent_path = tmp_dir.path().join("does_not_exist.txt");

        let mut tree = ProjectTree::new();
        let result = tree.add(&non_existent_path);
        assert!(result.is_err(), "Adding a non-existent file should return an error");
    }

    #[test]
    fn test_empty_directory_handling() {
        let tmp_dir = TempDir::new("empty_dir").unwrap();

        let mut tree = ProjectTree::new();
        tree.add(&tmp_dir.path()).unwrap();

        match tree.root {
            Node::Directory { ref children, .. } => assert!(children.is_empty(), "Empty directory should not add any children"),
            _ => panic!("Root should be a directory"),
        }
    }
}
