use std::io::prelude::*;

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};

/// Specifies the compression algorithm used for compressed asset data.
///
/// This enum is marked as non-exhaustive to allow adding new compression
/// algorithms in future versions without breaking changes.
///
/// Variant names are serialized in `snake_case` format (e.g., "zlib" for `Zlib`).
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CompressionMode {
    /// Zlib compression algorithm
    Zlib,
}
impl CompressionMode {
    /// Compresses the given bytes using this compression algorithm.
    ///
    /// Returns `Some(compressed_bytes)` on successful compression,
    /// or `None` if compression fails.
    ///
    /// # Arguments
    /// * `bytes` - The raw data to compress
    pub fn compress(&self, bytes: &[u8]) -> Option<Vec<u8>> {
        match self {
            Self::Zlib => {
                let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
                enc.write_all(bytes).ok()?;
                enc.finish().ok()
            }
        }
    }

    /// Decompresses the given bytes using this compression algorithm.
    ///
    /// Returns `Some(decompressed_bytes)` on successful decompression,
    /// or `None` if decompression fails or the data is corrupted.
    ///
    /// # Arguments
    /// * `bytes` - The compressed data to decompress
    pub fn decompress(&self, bytes: &[u8]) -> Option<Vec<u8>> {
        match self {
            Self::Zlib => {
                let mut dec = ZlibDecoder::new(bytes);
                let mut out = Vec::new();
                dec.read_to_end(&mut out).ok()?;
                Some(out)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn zlib_roundtrip() {
        // Make compressable data
        let values: [u8; 10] = rand::rng().random();
        let data = values
            .into_iter()
            .flat_map(|v| {
                let n: usize = rand::rng().random_range(16..48);
                std::iter::repeat_n(v, n)
            })
            .collect::<Vec<_>>();

        // Test compression
        let compressed = CompressionMode::Zlib.compress(&data).unwrap();
        assert!(compressed.len() < data.len());

        // Test decompression
        let decompress = CompressionMode::Zlib.decompress(&compressed).unwrap();
        assert_eq!(decompress, data);
    }
}
