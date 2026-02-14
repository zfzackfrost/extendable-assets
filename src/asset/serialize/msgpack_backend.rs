use crate::asset::{AssetSerializationBackend, SerializedAsset};

/// MessagePack-based asset serialization backend.
///
/// This backend uses rmp_serde to serialize and deserialize assets in MessagePack format.
/// MessagePack is a compact binary format that provides efficient serialization with
/// smaller output sizes and faster processing compared to text-based formats like JSON.
///
/// # Performance Characteristics
/// - Serialization: Fast speed, compact binary output
/// - Deserialization: Fast speed, efficient memory usage
/// - Storage: Space-efficient binary format, ideal for production environments
///
/// # Example
/// ```rust
/// # use extendable_assets::{AssetManager, FallbackFilesystem, MsgpackAssetSerializationBackend, AssetSerializationBackend, SerializedAsset};
/// # use std::sync::Arc;
/// # let fs = FallbackFilesystem::new(Vec::new());
/// # let mut asset_manager = AssetManager::new(Arc::new(fs));
///
/// let backend = MsgpackAssetSerializationBackend;
/// // Use backend for serialization operations
/// asset_manager.set_serialization_backend(Box::new(backend));
/// ```
pub struct MsgpackAssetSerializationBackend;

impl AssetSerializationBackend for MsgpackAssetSerializationBackend {
    /// Serializes a SerializedAsset to MessagePack bytes.
    ///
    /// Uses rmp_serde internally to convert the asset to a compact binary format.
    /// MessagePack provides excellent space efficiency and fast serialization performance.
    ///
    /// # Arguments
    /// * `asset` - The asset to serialize
    ///
    /// # Returns
    /// A vector of bytes containing the MessagePack representation, or an error if
    /// serialization fails (e.g., due to unsupported data types).
    fn serialize(&self, asset: &SerializedAsset) -> anyhow::Result<Vec<u8>> {
        // Use rmp_serde::to_vec for compact MessagePack binary output
        Ok(rmp_serde::to_vec(asset)?)
    }

    /// Deserializes MessagePack bytes back into a SerializedAsset.
    ///
    /// Parses the provided MessagePack bytes and reconstructs a SerializedAsset.
    /// The binary data must be valid MessagePack that matches the expected structure.
    ///
    /// # Arguments
    /// * `bytes` - MessagePack bytes to deserialize
    ///
    /// # Returns
    /// The reconstructed SerializedAsset, or an error if the data is corrupted
    /// or doesn't match the expected MessagePack structure.
    fn deserialize(&self, bytes: &[u8]) -> anyhow::Result<SerializedAsset> {
        // Parse MessagePack from byte slice directly for efficiency
        Ok(rmp_serde::from_slice(bytes)?)
    }
}
