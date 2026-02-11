use crate::asset::{AssetSerializationBackend, SerializedAsset};

/// JSON-based asset serialization backend.
///
/// This backend uses serde_json to serialize and deserialize assets in JSON format.
/// JSON is human-readable and widely supported, making it ideal for debugging,
/// configuration files, and interoperability with other systems.
///
/// # Performance Characteristics
/// - Serialization: Moderate speed, larger output size due to text format
/// - Deserialization: Moderate speed, good error reporting
/// - Storage: Human-readable but less space-efficient than binary formats
///
/// # Example
/// ```rust
/// # use extendable_assets::{AssetManager, FallbackFilesystem, JsonAssetSerializationBackend, AssetSerializationBackend, SerializedAsset};
/// # use std::sync::Arc;
/// # let fs = FallbackFilesystem::new(Vec::new());
/// # let mut asset_manager = AssetManager::new(Arc::new(fs));
///
/// let backend = JsonAssetSerializationBackend;
/// // Use backend for serialization operations
/// asset_manager.set_serialization_backend(Box::new(backend));
/// ```
pub struct JsonAssetSerializationBackend;

impl AssetSerializationBackend for JsonAssetSerializationBackend {
    /// Serializes a SerializedAsset to JSON bytes.
    ///
    /// Uses serde_json internally to convert the asset to a JSON byte vector.
    /// The resulting JSON will be compact (no pretty-printing) for efficiency.
    ///
    /// # Arguments
    /// * `asset` - The asset to serialize
    ///
    /// # Returns
    /// A vector of bytes containing the JSON representation, or an error if
    /// serialization fails (e.g., due to invalid UTF-8 in string fields).
    fn serialize(&self, asset: &SerializedAsset) -> anyhow::Result<Vec<u8>> {
        // Use serde_json::to_vec for compact JSON output
        Ok(serde_json::to_vec(asset)?)
    }

    /// Deserializes JSON bytes back into a SerializedAsset.
    ///
    /// Parses the provided JSON bytes and reconstructs a SerializedAsset.
    /// The JSON must match the expected SerializedAsset structure.
    ///
    /// # Arguments
    /// * `bytes` - JSON bytes to deserialize
    ///
    /// # Returns
    /// The reconstructed SerializedAsset, or an error if the JSON is malformed
    /// or doesn't match the expected structure.
    fn deserialize(&self, bytes: &[u8]) -> anyhow::Result<SerializedAsset> {
        // Parse JSON from byte slice directly for efficiency
        Ok(serde_json::from_slice(bytes)?)
    }
}
