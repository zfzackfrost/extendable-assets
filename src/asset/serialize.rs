use serde::{Deserialize, Serialize};

use crate::asset::AssetId;

/// A serialized representation of an asset for storage or transmission.
///
/// This struct contains all the necessary information to reconstruct an asset,
/// including its ID, type information, and the serialized data payload.
/// The data is stored as raw bytes and uses serde_bytes for efficient
/// serialization and deserialization.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct SerializedAsset {
    /// The unique identifier of the asset
    #[serde(default)]
    pub id: AssetId,
    /// The name of the asset type used to determine how to deserialize the data
    pub asset_type: String,
    /// The serialized asset data as raw bytes
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl PartialEq for SerializedAsset {
    /// Compares two SerializedAsset instances for equality.
    ///
    /// Two assets are considered equal if all their fields match exactly.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.asset_type == other.asset_type && self.data == other.data
    }
}

/// Trait for implementing different asset serialization backends.
///
/// This trait allows for pluggable serialization strategies for assets,
/// enabling support for different formats like JSON, MessagePack, bincode, etc.
/// Implementations must be thread-safe (Send + Sync).
pub trait AssetSerializationBackend: Send + Sync {
    /// Serializes a SerializedAsset into bytes using the backend's format.
    ///
    /// # Arguments
    /// * `asset` - The asset to serialize
    ///
    /// # Returns
    /// The serialized bytes on success, or an error if serialization fails
    fn serialize(&self, asset: &SerializedAsset) -> anyhow::Result<Vec<u8>>;

    /// Deserializes bytes back into a SerializedAsset using the backend's format.
    ///
    /// # Arguments
    /// * `bytes` - The serialized data to deserialize
    ///
    /// # Returns
    /// The deserialized SerializedAsset on success, or an error if deserialization fails
    fn deserialize(&self, bytes: &[u8]) -> anyhow::Result<SerializedAsset>;
}
/// A null implementation of AssetSerializationBackend that always returns errors.
///
/// This backend is useful as a placeholder or for testing error conditions.
/// All operations will fail with an "Unimplemented" error.
pub struct NullSerializationBackend;
impl AssetSerializationBackend for NullSerializationBackend {
    /// Always returns an error - this backend does not implement serialization.
    fn serialize(&self, _asset: &SerializedAsset) -> anyhow::Result<Vec<u8>> {
        Err(anyhow::anyhow!(
            "Unimplemented for NullSerializationBackend"
        ))
    }

    /// Always returns an error - this backend does not implement deserialization.
    fn deserialize(&self, _bytes: &[u8]) -> anyhow::Result<SerializedAsset> {
        Err(anyhow::anyhow!(
            "Unimplemented for NullSerializationBackend"
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::RngExt;
    use serde_test::{Token, assert_tokens};
    use std::sync::LazyLock;

    /// Static test data with 'static lifetime required by Token::Bytes.
    /// LazyLock provides thread-safe lazy initialization of random data.
    static DATA: LazyLock<Vec<u8>> = LazyLock::new(|| {
        let mut rng = rand::rng();
        let mut data = vec![0u8; 128];
        rng.fill(&mut data[..]);
        data
    });

    /// Tests SerializedAsset serde implementation
    #[test]
    fn serialized_asset_serde() {
        let mut rng = rand::rng();
        // Generate a random asset ID for testing
        let id: AssetId = rng.random();
        let asset_type: String = "TestAsset".into();

        let asset = SerializedAsset {
            id,
            asset_type: asset_type.clone(),
            data: DATA.clone(),
        };
        let tokens = &[
            Token::Struct {
                name: "SerializedAsset",
                len: 3,
            },
            Token::Str("id"),
            Token::U64(asset.id.into()),
            Token::Str("asset_type"),
            Token::String("TestAsset"),
            Token::Str("data"),
            // Token::Bytes requires &'static [u8], provided by static DATA
            Token::Bytes(&DATA[..]),
            Token::StructEnd,
        ];
        assert_tokens(&asset, tokens);
    }
}
