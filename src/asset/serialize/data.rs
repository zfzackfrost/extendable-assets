use crate::asset::serialize::CompressionMode;

/// Represents serialized asset data that can be either compressed or uncompressed.
///
/// This enum uses serde's untagged serialization, meaning the variant is determined
/// by the structure of the data rather than an explicit tag field.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum SerializedData {
    /// Compressed data variant containing compression mode and compressed bytes
    Compressed(
        /// The compression algorithm used to compress the data
        CompressionMode,
        /// The compressed data as a byte vector, serialized efficiently as bytes
        #[serde(with = "serde_bytes")]
        Vec<u8>,
    ),
    /// Uncompressed data variant containing raw bytes
    Uncompressed(
        /// The raw uncompressed data as a byte vector, serialized efficiently as bytes
        #[serde(with = "serde_bytes")]
        Vec<u8>,
    ),
}

impl SerializedData {
    /// Returns the compression mode if the data is compressed, or None if uncompressed.
    ///
    /// This method allows checking whether the serialized data uses compression
    /// and which compression algorithm was applied.
    pub fn compression_mode(&self) -> Option<CompressionMode> {
        match self {
            SerializedData::Compressed(mode, _) => Some(*mode),
            SerializedData::Uncompressed(_) => None,
        }
    }

    /// Returns a reference to the underlying data bytes.
    ///
    /// For compressed data, this returns the compressed bytes.
    /// For uncompressed data, this returns the raw bytes.
    pub fn data(&self) -> &[u8] {
        match self {
            SerializedData::Compressed(_, data) => data,
            SerializedData::Uncompressed(data) => data,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use serde_json::json;

    /// Tests that compressed data serializes as a tuple with compression mode and data
    #[test]
    fn compressed_enum_repr() {
        // Generate random test data
        let mut bytes = vec![0u8; 128];
        rand::rng().fill(&mut bytes[..]);

        // Create compressed data variant
        let data = SerializedData::Compressed(CompressionMode::Zlib, bytes.clone());
        // Expected JSON format: [compression_mode, data]
        let expected = json!([CompressionMode::Zlib, bytes]);
        let serialized = serde_json::to_value(data).unwrap();
        assert_eq!(expected, serialized);
    }

    /// Tests that uncompressed data serializes as just the raw bytes
    #[test]
    fn uncompressed_enum_repr() {
        // Generate random test data
        let mut bytes = vec![0u8; 128];
        rand::rng().fill(&mut bytes[..]);

        // Create uncompressed data variant
        let data = SerializedData::Uncompressed(bytes.clone());
        // Expected JSON format: just the bytes array
        let expected = json!(bytes);
        let serialized = serde_json::to_value(data).unwrap();
        assert_eq!(expected, serialized);
    }
}
