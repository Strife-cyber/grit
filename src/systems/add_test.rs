use std::env;
use std::fs::{self};
use std::io::{self};
use crate::systems::init::{init_grit};

/// Helper function to set up a test `.grit` repository
fn setup_grit_repo() -> io::Result<()> {
    // Initialize a .grit repository
    init_grit()?;
    // Ensure the .grit directory exists
    let grit_path = env::current_dir()?.join(".grit");
    if !grit_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Failed to create .grit directory"));
    }
    Ok(())
}

/// Helper function to clean up after tests
fn cleanup_grit_repo() -> io::Result<()> {
    let grit_path = env::current_dir()?.join(".grit");
    if grit_path.exists() {
        fs::remove_dir_all(grit_path)?;  // Clean up .grit directory
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::structure::serialization::load;
    use super::*;

    #[test]
    fn test_add_file_to_grit_repo() -> io::Result<()> {
        // Setup .grit directory
        setup_grit_repo()?;

        // Create a dummy file to add
        let file_path = Path::new("test_file.txt");
        let mut file = File::create(file_path)?;
        writeln!(file, "This is a test file.")?;

        // Add the file to the repository using the `add` function
        add(Some(file_path.to_str().unwrap()))?;

        // Verify if the file was added correctly to the project tree
        let tree = load(None)?;
        let files = tree.list_files();

        // Check if the file is in the tree (this assumes ProjectTree has a method to check for files)
        assert!(files.len() > 0);
        assert!(files.contains(&PathBuf::from(file_path.to_str().unwrap())));

        // Cleanup after the test
        cleanup_grit_repo()?;

        // Clean up the test file
        fs::remove_file(file_path)?;

        Ok(())
    }

    #[test]
    fn test_add_multiple_files_to_grit_repo() -> io::Result<()> {
        // Setup .grit directory
        setup_grit_repo()?;

        // Create dummy files to add
        let file1_path = Path::new("test_file1.txt");
        let file2_path = Path::new("test_file2.txt");
        let mut file1 = File::create(file1_path)?;
        let mut file2 = File::create(file2_path)?;
        writeln!(file1, "Test content 1")?;
        writeln!(file2, "Test content 2")?;

        // Add both files to the repository
        add(None)?; // No specific path, so all files should be added

        // Verify both files were added correctly to the project tree
        let tree = load(None)?;

        // Check if both files are in the tree
        assert!(tree.exists(file1_path));
        assert!(tree.exists(file2_path));

        // Cleanup after the test
        cleanup_grit_repo()?;

        // Clean up the test files
        fs::remove_file(file1_path)?;
        fs::remove_file(file2_path)?;

        Ok(())
    }
}
