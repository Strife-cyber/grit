use super::init::{
    GRIT_DIR, CONFIG_FILE,
    init_grit, find_grit_root,
    normalize_path, is_grit_repo,
    update_grit_root
};

#[cfg(test)]
mod init_tests {
    use std::fs;
    use super::*;
    use std::io::{Read};
    use tempfile::tempdir;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_init_grit_creates_grit_directory() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let grit_path = temp_dir.path().join(GRIT_DIR);

        assert!(!grit_path.exists(), "Grit directory should not exist before initialization");

        init_grit().unwrap();

        assert!(grit_path.exists(), "Grit directory should be created after initialization");
        assert!(grit_path.is_dir(), ".grit should be a directory");
    }

    #[test]
    fn test_init_grit_creates_config_file_with_correct_path() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        init_grit().unwrap();

        let config_path = temp_dir.path().join(GRIT_DIR).join(CONFIG_FILE);

        assert!(config_path.exists(), "Config file should be created");

        let mut file = fs::File::open(&config_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let expected_path = temp_dir.path().canonicalize().unwrap();
        let actual_path = PathBuf::from(contents.trim()).canonicalize().unwrap();

        assert_eq!(actual_path, expected_path, "Config file path mismatch");
    }

    #[test]
    fn test_find_grit_root_returns_correct_path() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        init_grit().unwrap();
        let grit_root = find_grit_root(temp_dir.path()).unwrap();
        let dir = normalize_path(temp_dir.path());
        assert_eq!(
            grit_root.canonicalize().unwrap(),
            dir.canonicalize().unwrap(),
        );
    }

    #[test]
    fn test_find_grit_root_returns_none_if_not_in_repo() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        assert!(find_grit_root(temp_dir.path()).is_none(), "Should return None if not inside a grit repo");
    }

    #[test]
    fn test_is_grit_repo_detects_repo_correctly() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        assert!(!is_grit_repo(), "Should return false before initializing");

        init_grit().unwrap();

        assert!(is_grit_repo(), "Should return true after initializing");
    }

    #[test]
    fn test_update_grit_root_updates_config_path() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        init_grit().unwrap();

        let config_path = temp_dir.path().join(GRIT_DIR).join(CONFIG_FILE);
        let mut file = fs::File::open(&config_path).unwrap();
        let mut old_contents = String::new();
        file.read_to_string(&mut old_contents).unwrap();
        drop(file);

        // Simulate moving the project
        let new_temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&new_temp_dir).unwrap();
        let new_grit_path = new_temp_dir.path().join(GRIT_DIR);
        fs::rename(temp_dir.path().join(GRIT_DIR), &new_grit_path).unwrap();

        update_grit_root(new_temp_dir.path()).unwrap();

        let new_config_path = new_grit_path.join(CONFIG_FILE);
        let mut new_file = fs::File::open(&new_config_path).unwrap();
        let mut new_contents = String::new();
        new_file.read_to_string(&mut new_contents).unwrap();

        assert_ne!(
            old_contents.trim(),
            new_contents.trim(),
            "Config file should be updated with new path"
        );
        let expected_path = new_temp_dir.path().canonicalize().unwrap();
        let actual_path = Path::new(new_contents.trim()).canonicalize().unwrap();

        assert_eq!(
            actual_path, expected_path,
            "Updated config path does not match expected new path"
        );
    }
}
