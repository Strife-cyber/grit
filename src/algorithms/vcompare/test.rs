use super::edit::Edit;
use super::difference::myers;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to assert the changes in a readable format
    fn assert_changes(expected_changes: Vec<Edit>, actual_changes: Vec<Edit>) {
        assert_eq!(expected_changes.len(), actual_changes.len());
        for (expected, actual) in expected_changes.iter().zip(actual_changes.iter()) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_basic_diff() {
        let old_version = vec![
            "Line 1".to_string(),
            "Hello World".to_string(),
            "Line 3".to_string(),
        ];
        let new_version = vec![
            "Line 1".to_string(),
            "Hello Git".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
        ];

        let changes = myers(old_version, new_version);

        let expected_changes = vec![
            Edit::Replace(1, "Hello Git".to_string()),
            Edit::Insert(3, "Line 4".to_string())
        ];

        assert_changes(expected_changes, changes);
    }

    #[test]
    fn test_no_changes() {
        let old_version = vec![
            "Line 1".to_string(),
            "Hello World".to_string(),
            "Line 3".to_string(),
        ];
        let new_version = vec![
            "Line 1".to_string(),
            "Hello World".to_string(),
            "Line 3".to_string(),
        ];

        let changes = myers(old_version, new_version);
        assert!(changes.is_empty());
    }

    #[test]
    fn test_only_deletions() {
        let old_version = vec![
            "Line 1".to_string(),
            "Hello World".to_string(),
            "Line 3".to_string(),
        ];
        let new_version = vec![
            "Line 1".to_string(),
        ];

        let changes = myers(old_version, new_version);

        let expected_changes = vec![
            Edit::Delete(1),
            Edit::Delete(2)
        ];

        assert_changes(expected_changes, changes);
    }

    #[test]
    fn test_only_insertions() {
        let old_version = vec![
            "Line 1".to_string(),
        ];
        let new_version = vec![
            "Line 1".to_string(),
            "Hello World".to_string(),
            "Line 3".to_string(),
        ];

        let changes = myers(old_version, new_version);

        let expected_changes = vec![
            Edit::Insert(1, "Hello World".to_string()),
            Edit::Insert(2, "Line 3".to_string())
        ];

        assert_changes(expected_changes, changes);
    }
}
