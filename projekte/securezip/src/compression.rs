use flate2::Compression;
use anyhow::Result;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{Read, Write};

// [David]: Implement compression logic.
// Function signature: `compress(data: &[u8]) -> Result<Vec<u8>>`
// Use `flate2::write::GzEncoder` or similar.
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;
    Ok(compressed)
}

// [David]: Implement decompression logic.
// Function signature: `decompress(data: &[u8]) -> Result<Vec<u8>>`
// Use `flate2::read::GzDecoder` or similar.
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}
