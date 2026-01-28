use crate::asset::AssetData;

use thiserror::Error;

/// Errors that can occur during asset saving.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AssetSaveError {
    /// The asset type is not supported by this saver.
    #[error("Unsupported asset type")]
    UnsupportedType,

    /// Failed to serialize the asset data to bytes.
    #[error("Serialization error: {0}")]
    Serialization(anyhow::Error),

    /// Any other error that occurred during saving.
    #[error(transparent)]
    Other(anyhow::Error),
}

/// Trait for saving assets to raw byte data.
/// 
/// Asset savers are responsible for converting typed asset data back into raw bytes
/// that can be written to files or transmitted over the network.
pub trait AssetSaver {
    /// Converts asset data into raw bytes.
    /// 
    /// # Arguments
    /// 
    /// * `asset` - The asset data to serialize
    /// 
    /// # Returns
    /// 
    /// The serialized byte data on success, or an error if saving failed.
    fn asset_to_bytes(&self, asset: &dyn AssetData) -> Result<Vec<u8>, AssetSaveError>;
}
