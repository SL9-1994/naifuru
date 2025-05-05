use std::{fs::File, io::Read, path::Path};

use crate::analysis_config::TextOrBinary;

pub fn read_text(path: &Path) -> Result<String, std::io::Error> {
    let config: String = std::fs::read_to_string(path)?;

    Ok(config)
}

pub fn read_text_as_lines(path: &Path) -> Result<TextOrBinary, std::io::Error> {
    let config: String = std::fs::read_to_string(path)?;
    let lines: Vec<String> = config.lines().map(|line| line.to_string()).collect();

    Ok(TextOrBinary::Text(lines))
}

pub fn read_binary(file_path: &Path) -> Result<TextOrBinary, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(TextOrBinary::Binary(buffer))
}

pub fn to_fixed_chunks<const N: usize>(data: Vec<u8>) -> Vec<[u8; N]> {
    data.chunks_exact(N)
        .map(|chunk| {
            let arr: [u8; N] = chunk.try_into().expect("Chunk size mismatch");
            arr
        })
        .collect()
}
