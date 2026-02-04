mod data;
mod serialize;

pub use data::*;
pub use serialize::*;

use std::sync::Weak;

use crate::asset_type::AssetType;
use crate::loader::AssetLoadError;
use crate::manager::AssetManager;

/// Unique identifier for assets in the system.
pub type AssetId = u64;

/// Errors that can occur during asset operations.
///
/// This enum represents all possible errors that can happen when working with assets,
/// including loading, type resolution, and general asset manipulation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AssetError {
    /// Error occurred during asset loading from bytes.
    ///
    /// This variant wraps loader-specific errors that can happen when attempting
    /// to deserialize asset data from raw bytes using an asset type's loader.
    #[error("Error loading asset data from bytes: {0}")]
    Loader(#[from] AssetLoadError),

    /// The asset type reference has been dropped and is no longer available.
    ///
    /// This error occurs when trying to access an asset type through a weak reference
    /// that can no longer be upgraded because the asset type has been dropped from memory.
    #[error("A weak pointer to an AssetType could not be upgraded")]
    TypeDropped,

    /// The specified asset type could not be found in the asset manager.
    ///
    /// This error occurs when attempting to look up an asset type by name that
    /// is not registered in the asset manager.
    #[error("Asset type was not found: {0}")]
    TypeNotFound(String),

    /// Any other error that doesn't fit into the specific categories above.
    ///
    /// This is a catch-all variant for wrapping other error types that may
    /// occur during asset operations.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// An asset containing type information and data.
///
/// Assets are the core data containers in the asset system. Each asset has
/// a unique ID, an asset type that defines how to load and save it, and the
/// actual data payload.
pub struct Asset {
    /// Unique identifier for this asset
    id: AssetId,
    /// Type definition that specifies how to handle this asset
    asset_type: Weak<dyn AssetType>,
    /// The actual asset data
    data: Box<dyn AssetData>,
}
impl Asset {
    /// Creates a new asset with the given type and data.
    ///
    /// The asset ID is initially set to 0 and will be assigned by the asset manager when the asset is registered.
    ///
    /// # Arguments
    ///
    /// * `asset_type` - The type definition for this asset
    /// * `data` - The actual asset data
    #[inline]
    pub fn new(asset_type: Weak<dyn AssetType>, data: Box<dyn AssetData>) -> Self {
        Self {
            // ID will be set by the asset manager during registration
            id: 0,
            asset_type,
            data,
        }
    }
    /// Creates an asset from a serialized representation.
    ///
    /// This method reconstructs an asset from its serialized form by looking up
    /// the asset type from the manager and deserializing the data using the
    /// appropriate loader.
    ///
    /// # Arguments
    ///
    /// * `mgr` - The asset manager to use for type lookup
    /// * `serialized` - The serialized asset data
    ///
    /// # Returns
    ///
    /// `Ok(Asset)` if deserialization succeeds, `Err(AssetError)` if the asset type
    /// is not found or deserialization fails.
    pub fn from_serialized(
        mgr: &AssetManager,
        serialized: SerializedAsset,
    ) -> Result<Self, AssetError> {
        // Extract the asset ID from serialized data
        let id = serialized.id;
        // Look up the asset type by name in the manager
        let asset_type = mgr
            .asset_type_by_name(&serialized.asset_type)
            .ok_or_else(|| AssetError::TypeNotFound(serialized.asset_type))?;
        // Upgrade the weak reference to ensure the asset type is still valid
        let asset_type_ptr = asset_type.upgrade().ok_or(AssetError::TypeDropped)?;
        // Get the serialized data bytes
        let data_bytes = serialized.data;
        // Use the asset type's loader to deserialize the data
        let data = asset_type_ptr
            .loader()
            .asset_from_bytes(&data_bytes)
            .map_err(AssetError::from)?;
        Ok(Self {
            id,
            asset_type,
            data,
        })
    }
    /// Returns the unique identifier for this asset.
    #[inline]
    pub fn id(&self) -> AssetId {
        self.id
    }
    /// Sets the asset's unique identifier.
    ///
    /// This is only used internally by the asset manager during registration.
    #[inline]
    pub(crate) fn set_id(&mut self, new_id: AssetId) {
        self.id = new_id;
    }
    /// Returns a clone of the asset type definition.
    #[inline]
    pub fn asset_type(&self) -> Weak<dyn AssetType> {
        Weak::clone(&self.asset_type)
    }
    /// Returns a reference to the asset's data.
    ///
    /// The data can be downcast to the specific type using the downcast methods
    /// provided by the `DowncastSync` super-trait.
    #[inline]
    pub fn data(&self) -> &dyn AssetData {
        self.data.as_ref()
    }
}
