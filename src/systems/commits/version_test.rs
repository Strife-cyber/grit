use super::versioning::Version;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FILE: &str = "test_file.txt";
    const TEST_JSON: &str = "test_versions.json";

    /// Helper function to clean up test files
    fn cleanup() {
        let _ = fs::remove_file(TEST_FILE);
        let _ = fs::remove_file(TEST_JSON);
    }

    #[test]
    fn test_create_version_file() {
        cleanup();
        fs::write(TEST_FILE, "Hello, world!\nThis is version 0.\n").unwrap();

        let result = Version::create(TEST_FILE, TEST_JSON);
        assert!(result.is_ok(), "Failed to create version file");

        let version_data = Version::load(TEST_JSON).unwrap();
        assert_eq!(version_data.original, "Hello, world!\nThis is version 0.\n");
        assert!(version_data.versions.is_empty(), "New file should have no versions");
    }

    #[test]
    fn test_add_version() {
        cleanup();
        fs::write(TEST_FILE, "Hello, world!\nThis is version 0.\n").unwrap();
        let mut version_data = Version::load(TEST_JSON).unwrap_or_else(|_| {
            Version::create(TEST_FILE, TEST_JSON).unwrap();
            Version::load(TEST_JSON).unwrap()
        });

        // Modify file content
        fs::write(TEST_FILE, "Hello, universe!\nThis is version 1.\n").unwrap();
        version_data.add_version(TEST_FILE, TEST_JSON).unwrap();
        assert_eq!(version_data.versions.len(), 1, "Should have one version entry");

        // Check that the latest reconstruction matches the new content
        let latest_version = version_data.reconstruct_latest();
        assert_eq!(latest_version, "Hello, universe!\nThis is version 1.");
    }

    #[test]
    fn test_reconstruct_version() {
        cleanup();
        fs::write(TEST_FILE, "Hello, world!\nThis is version 0.\n").unwrap();
        let mut version_data = Version::load(TEST_JSON).unwrap_or_else(|_| {
            Version::create(TEST_FILE, TEST_JSON).unwrap();
            Version::load(TEST_JSON).unwrap()
        });

        // Modify file and add a new version
        fs::write(TEST_FILE, "Hello, universe!\nThis is version 1.\n").unwrap();
        version_data.add_version(TEST_FILE, TEST_JSON).unwrap();
        let version_id = version_data.versions[0].version_id.clone();

        // Further modify file and add another version
        fs::write(TEST_FILE, "Hello, universe!\nThis is version 2.\n").unwrap();
        version_data.add_version(TEST_FILE, TEST_JSON).unwrap();

        assert_eq!(version_data.versions.len(), 2, "Should have two versions");

        // Retrieve first version
        let reconstructed_v1 = version_data.reconstruct_version(&version_id);
        assert!(reconstructed_v1.is_some(), "Version 1 should exist");
        assert_eq!(reconstructed_v1.unwrap(), "Hello, universe!\nThis is version 1.");
    }

    #[test]
    fn test_no_changes_detected() {
        cleanup();
        fs::write(TEST_FILE, "Hello, world!\nThis is version 0.\n").unwrap();
        let mut version_data = Version::load(TEST_JSON).unwrap_or_else(|_| {
            Version::create(TEST_FILE, TEST_JSON).unwrap();
            Version::load(TEST_JSON).unwrap()
        });

        version_data.add_version(TEST_FILE, TEST_JSON).unwrap();
        assert_eq!(version_data.versions.len(), 0, "No new version should be created if no changes");

        // Modify file and then revert it back
        fs::write(TEST_FILE, "Hello, world!\nThis is version 1.\n").unwrap();
        version_data.add_version(TEST_FILE, TEST_JSON).unwrap();
        fs::write(TEST_FILE, "Hello, world!\nThis is version 0.\n").unwrap();
        version_data.add_version(TEST_FILE, TEST_JSON).unwrap();

        assert_eq!(version_data.versions.len(), 2, "Only one version should be stored after a revert");
    }
}
