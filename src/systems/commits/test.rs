use super::commit::Commit;
use super::functions::{
    HEAD_FILE,
    save_commit, load_commit,
    load_all_commits, get_head_commit
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::{env, fs, io};
    use crate::systems::add::add;
    use std::path::{Path, PathBuf};
    use crate::systems::init::init_grit;

    fn setup() -> io::Result<()> {
        // Initialize a .grit repository
        init_grit()?;
        // Ensure the .grit directory exists
        let grit_path = env::current_dir()?.join(".grit");
        if !grit_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Failed to create .grit directory"));
        }
        Ok(())
    }

    fn create_and_modify_file() -> io::Result<PathBuf> {
        setup()?;

        // Create a dummy file
        let file_path = Path::new("test_file.txt").to_path_buf();
        let mut file = File::create(&file_path)?;
        writeln!(file, "This is the initial content.")?;
        file.sync_all()?;

        // Add the file (first commit)
        add(Some(file_path.to_str().unwrap()))?;
        let commit1 = Commit::new("Initial Commit", "Tester")?;
        save_commit(&commit1.unwrap())?;

        // Modify the file
        let mut file = File::options().append(true).open(&file_path)?;
        writeln!(file, "This is a modified line.")?;
        file.sync_all()?;

        // Add the modified file (second commit)
        add(Some(file_path.to_str().unwrap()))?;
        Ok(file_path)
    }

    #[test]
    fn test_commit_creation_no_changes() {
        setup().unwrap();
        let commit = Commit::new("Test Commit", "Author").unwrap();
        assert!(commit.is_none(), "Commit should not be created if there are no changes.");
    }

    #[test]
    fn test_commit_creation_with_changes() {
        let file_path = create_and_modify_file().unwrap();

        let commit = Commit::new("Modified Commit", "Author").unwrap();
        assert!(commit.is_some(), "Commit should be created when there are modifications.");
        let commit = commit.unwrap();

        assert!(!commit.id.is_empty(), "Commit ID should not be empty.");
        assert!(!commit.files.is_empty(), "Commit should track modified files.");
        assert!(commit.files.contains(&file_path.to_str().unwrap().to_string()), "Commit should include modified file.");
    }

    #[test]
    fn test_save_commit_and_retrieve() {
        let commit = Commit::new("Initial Commit", "Tester").unwrap().unwrap();

        // Save commit
        save_commit(&commit).unwrap();

        // Check if the commit is saved correctly
        let saved_commits = load_all_commits().unwrap();
        assert!(saved_commits.contains_key(&commit.id), "Commit should be saved in commits.json");

        // Check HEAD update
        let head_commit = get_head_commit().unwrap();
        assert_eq!(head_commit, Some(commit.id.clone()), "HEAD should point to latest commit");
    }

    #[test]
    fn test_load_commit_by_id() {
        let commit = Commit::new("Feature Commit", "Developer").unwrap().unwrap();

        // Save the commit
        save_commit(&commit).unwrap();

        // Load the commit back
        let loaded_commit = load_commit(&commit.id).unwrap();
        assert!(loaded_commit.is_some(), "Commit should be found");
        assert_eq!(loaded_commit.unwrap().id, commit.id, "Loaded commit should match the original");
    }

    #[test]
    fn test_get_head_commit_no_commits() {
        // Ensure HEAD file does not exist
        fs::remove_file(HEAD_FILE).ok();
        let head_commit = get_head_commit().unwrap();
        assert!(head_commit.is_none(), "HEAD should be None if no commits exist");
    }
}
