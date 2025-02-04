use super::functions::read_file;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn create_test_file(file_path: &str, content: &[u8]) {
        let mut file = File::create(file_path).expect("Failed to create test file");
        file.write_all(content).expect("Failed to write to test file");
    }

    #[test]
    fn test_read_utf8_file() {
        let file_path = "test_utf8.txt";
        let content = "Hello, world! 你好，世界！";
        create_test_file(file_path, content.as_bytes());

        let result = read_file(file_path).expect("Failed to read file");
        assert_eq!(result, content, "UTF-8 file should be read correctly");

        std::fs::remove_file(file_path).ok(); // Cleanup
    }

    #[test]
    fn test_read_non_utf8_file() {
        let file_path = "test_non_utf8.txt";
        let content = vec![0xC3, 0x28, 0xB1, 0x39]; // Invalid UTF-8 bytes
        create_test_file(file_path, &content);

        let result = read_file(file_path).expect("Failed to read file");
        assert!(!result.is_empty(), "Non-UTF-8 file should return some readable content");

        std::fs::remove_file(file_path).ok(); // Cleanup
    }

    #[test]
    fn test_read_iso_8859_1_file() {
        let file_path = "test_iso8859.txt";
        let content = vec![0xC9, 0xE9, 0xE0, 0xF4]; // "Ééàô" in ISO-8859-1
        create_test_file(file_path, &content);

        let result = read_file(file_path).expect("Failed to read file");
        assert!(!result.is_empty(), "ISO-8859-1 file should return some readable content");

        std::fs::remove_file(file_path).ok(); // Cleanup
    }

    #[test]
    fn test_read_empty_file() {
        let file_path = "test_empty.txt";
        create_test_file(file_path, b""); // Create an empty file

        let result = read_file(file_path).expect("Failed to read file");
        assert!(result.is_empty(), "Empty file should return an empty string");

        std::fs::remove_file(file_path).ok(); // Cleanup
    }
}
