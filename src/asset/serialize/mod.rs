mod backend;
pub use backend::*;

#[cfg(feature = "backend-json")]
mod json_backend;
#[cfg(feature = "backend-json")]
pub use json_backend::*;

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
