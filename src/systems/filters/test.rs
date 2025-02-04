use super::filter::{
    FileFilter, load_file_filter, filter_paths
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    /// Helper function to create a temporary filter file
    fn create_temp_filter(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file.write_all(content.as_bytes()).expect("Failed to write to temp file");
        temp_file
    }

    #[test]
    fn test_load_file_filter_toml() {
        let toml_content = r#"
            allowed = ["/home/user/docs", "/home/user/projects"]
            denied = ["/home/user/private"]
        "#;

        let temp_file = create_temp_filter(toml_content);
        let filter = load_file_filter(temp_file.path().to_str().unwrap()).expect("Failed to load TOML filter");

        assert_eq!(filter.allowed, Some(vec![
            "/home/user/docs".to_string(),
            "/home/user/projects".to_string(),
        ]));
        assert_eq!(filter.denied, Some(vec!["/home/user/private".to_string()]));
    }

    #[test]
    fn test_load_file_filter_json() {
        let json_content = r#"
        {
            "allowed": ["/home/user/docs", "/home/user/projects"],
            "denied": ["/home/user/private"]
        }
        "#;

        let temp_file = create_temp_filter(json_content);
        let filter = load_file_filter(temp_file.path().to_str().unwrap()).expect("Failed to load JSON filter");

        assert_eq!(filter.allowed, Some(vec![
            "/home/user/docs".to_string(),
            "/home/user/projects".to_string(),
        ]));
        assert_eq!(filter.denied, Some(vec!["/home/user/private".to_string()]));
    }

    #[test]
    fn test_filter_paths_allowed_only() {
        let filter = FileFilter {
            allowed: Some(vec![
                "/home/user/docs".to_string(),
                "/home/user/projects".to_string(),
            ]),
            denied: None,
        };

        let paths = vec![
            PathBuf::from("/home/user/docs/1"),
            PathBuf::from("/home/user/private/1"),
            PathBuf::from("/home/user/projects/1"),
        ];

        let filtered = filter_paths(paths, &filter);
        assert_eq!(filtered, vec![
            PathBuf::from("/home/user/docs/1"),
            PathBuf::from("/home/user/projects/1"),
        ]);
    }

    #[test]
    fn test_filter_paths_denied_only() {
        let filter = FileFilter {
            allowed: None,
            denied: Some(vec!["/home/user/private".to_string()]),
        };

        let paths = vec![
            PathBuf::from("/home/user/docs"),
            PathBuf::from("/home/user/private/file.txt"),
            PathBuf::from("/home/user/projects"),
        ];

        let filtered = filter_paths(paths, &filter);
        assert_eq!(filtered, vec![
            PathBuf::from("/home/user/docs"),
            PathBuf::from("/home/user/projects"),
        ]);
    }

    #[test]
    fn test_filter_paths_allowed_and_denied() {
        let filter = FileFilter {
            allowed: Some(vec![
                "/home/user/docs".to_string(),
                "/home/user/projects".to_string(),
            ]),
            denied: Some(vec!["/home/user/docs".to_string()]), // Docs should be removed
        };

        let paths = vec![
            PathBuf::from("/home/user/docs"),
            PathBuf::from("/home/user/private"),
            PathBuf::from("/home/user/projects"),
        ];

        let filtered = filter_paths(paths, &filter);
        assert_eq!(filtered, vec![
            PathBuf::from("/home/user/projects"),
        ]);
    }

    #[test]
    fn test_filter_paths_no_restrictions() {
        let filter = FileFilter {
            allowed: None,
            denied: None,
        };

        let paths = vec![
            PathBuf::from("/home/user/docs"),
            PathBuf::from("/home/user/private"),
            PathBuf::from("/home/user/projects"),
        ];

        let filtered = filter_paths(paths.clone(), &filter);
        assert_eq!(filtered, paths); // Should return all paths
    }
}
