use std::fs::File;
use std::path::Path;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::{self, Read, Write};

pub fn compress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = io::BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut writer = ZlibEncoder::new(output_file, Compression::best());

    // Use a larger buffer for better performance
    let mut buffer = vec![0; 8192]; // 8 KB buffer

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&buffer[..bytes_read])?;
    }

    // Finalize the compression and ensure all data is written to the output file
    writer.finish()?;
    Ok(())
}

// Helper function to decompress the compressed file and compare it with the original
pub fn decompress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut decoder = ZlibDecoder::new(input_file);
    let mut output_file = File::create(output_path)?;
    io::copy(&mut decoder, &mut output_file)?;
    Ok(())
}