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
    pub id: AssetId,
    /// The name of the asset type used to determine how to deserialize the data
    pub asset_type: String,
    /// The serialized asset data as raw bytes
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
