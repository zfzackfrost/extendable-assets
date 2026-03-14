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
        let _ = bytes;
        todo!("Implement compress")
    }

    /// Decompresses the given bytes using this compression algorithm.
    ///
    /// Returns `Some(decompressed_bytes)` on successful decompression,
    /// or `None` if decompression fails or the data is corrupted.
    ///
    /// # Arguments
    /// * `bytes` - The compressed data to decompress
    pub fn decompress(&self, bytes: &[u8]) -> Option<Vec<u8>> {
        let _ = bytes;
        todo!("Implement decompress")
    }
}
