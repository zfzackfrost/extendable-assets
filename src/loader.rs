use std::sync::Arc;

use crate::asset::AssetData;
use crate::manager::AssetManagerContext;

use thiserror::Error;

/// Errors that can occur during asset loading.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AssetLoadError {
    /// Failed to deserialize the asset data from bytes.
    #[error("Deserialization error: {0}")]
    Deserialization(anyhow::Error),

    /// Any other error that occurred during loading.
    #[error(transparent)]
    Other(anyhow::Error),
}

/// Trait for loading assets from raw byte data.
///
/// Asset loaders are responsible for converting raw bytes (typically read from files)
/// into typed asset data that can be used by the application.
pub trait AssetLoader {
    /// Converts raw bytes into asset data.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The raw byte data to deserialize
    ///
    /// # Returns
    ///
    /// The loaded asset data on success, or an error if loading failed.
    fn asset_from_bytes(
        &self,
        bytes: &[u8],
        context: Option<Arc<dyn AssetManagerContext>>,
    ) -> Result<Box<dyn AssetData>, AssetLoadError>;
}
