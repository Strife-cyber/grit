use std::io;
use std::path::Path;

pub(super) mod compress;
mod test;

#[allow(dead_code)]
pub fn comp(input_path: &Path, output_path: &Path) -> io::Result<()> {
    compress::compress_file(input_path, output_path)
}
