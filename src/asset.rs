use std::sync::Arc;

use downcast_rs::{DowncastSync, impl_downcast};

use crate::asset_type::AssetType;

/// Unique identifier for assets in the system.
pub type AssetId = u64;

/// An asset containing type information and data.
///
/// Assets are the core data containers in the asset system. Each asset has
/// a unique ID, an asset type that defines how to load and save it, and the
/// actual data payload.
pub struct Asset {
    /// Unique identifier for this asset
    id: AssetId,
    /// Type definition that specifies how to handle this asset
    asset_type: Arc<dyn AssetType>,
    /// The actual asset data
    data: Box<dyn AssetData>,
}
impl Asset {
    /// Creates a new asset with the given type and data.
    ///
    /// The asset ID is initially set to 0 and will be assigned by the asset manager
    /// when the asset is registered.
    ///
    /// # Arguments
    ///
    /// * `asset_type` - The type definition for this asset
    /// * `data` - The actual asset data
    #[inline]
    pub fn new(asset_type: Arc<dyn AssetType>, data: Box<dyn AssetData>) -> Self {
        Self {
            // ID will be set by the asset manager during registration
            id: 0,
            asset_type,
            data,
        }
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
    pub fn asset_type(&self) -> Arc<dyn AssetType> {
        Arc::clone(&self.asset_type)
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

/// Trait for asset data that can be stored in an asset.
///
/// This trait extends `DowncastSync` to allow for safe downcasting to concrete types.
/// Types must explicitly implement this trait to be used as asset data.
pub trait AssetData: DowncastSync {}
impl_downcast!(sync AssetData);
