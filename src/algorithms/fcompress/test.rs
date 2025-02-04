use super::compress::{compress_file, decompress_file};

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;
    use std::fs::File;
    use tempfile::NamedTempFile;
    use std::io::{Read, Seek, SeekFrom, Write};

    #[test]
    fn test_compress_file() -> io::Result<()> {
        // Create a temporary file for the input
        let mut input_file = NamedTempFile::new()?;
        let input_data = b"Line 1\nHello World\nLine 3\n";
        input_file.write_all(input_data)?;

        // Rewind the input file to the beginning so it can be read again
        input_file.seek(SeekFrom::Start(0))?;

        // Create a temporary file for the compressed output
        let compressed_file = NamedTempFile::new()?;

        // Call the compress_file function
        compress_file(input_file.path(), compressed_file.path())?;

        // Check if the compressed file exists and is not empty
        let metadata = std::fs::metadata(compressed_file.path())?;
        assert!(metadata.len() > 0);

        // Create a temporary file for the decompressed output
        let decompressed_file = NamedTempFile::new()?;

        // Decompress the file
        decompress_file(compressed_file.path(), decompressed_file.path())?;

        // Compare the decompressed file with the original input file
        let mut original = Vec::new();
        let mut decompressed = Vec::new();

        // Rewind the input file to the beginning before reading
        input_file.seek(SeekFrom::Start(0))?;
        input_file.read_to_end(&mut original)?;

        File::open(decompressed_file.path())?.read_to_end(&mut decompressed)?;

        assert_eq!(original, decompressed);

        Ok(())
    }
}
