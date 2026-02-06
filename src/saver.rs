use std::sync::Arc;

use crate::asset::AssetData;
use crate::manager::AssetManagerContext;

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
    /// This method is called by the asset manager when an asset needs to be
    /// saved to persistent storage or transmitted over the network.
    ///
    /// # Arguments
    ///
    /// * `asset` - The asset data to serialize into bytes
    /// * `context` - Optional context providing access to asset manager state
    ///   and configuration during the saving process
    ///
    /// # Returns
    ///
    /// The serialized byte data on success, or an error if saving failed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The asset type is not supported by this saver
    /// * Serialization of the asset data fails
    /// * Any other error occurs during the saving process
    fn asset_to_bytes(
        &self,
        asset: &dyn AssetData,
        context: Option<Arc<dyn AssetManagerContext>>,
    ) -> Result<Vec<u8>, AssetSaveError>;
}
