use crate::asset::serialize::SerializedAsset;

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
