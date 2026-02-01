mod data;
mod serialize;

pub use data::*;
pub use serialize::*;

use std::sync::Weak;

use crate::{AssetManager, asset_type::AssetType};

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
    asset_type: Weak<dyn AssetType>,
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
    /// `Some(Asset)` if deserialization succeeds, `None` if the asset type
    /// is not found or deserialization fails.
    pub fn from_serialized(mgr: &AssetManager, serialized: SerializedAsset) -> Option<Self> {
        // Extract the asset ID from serialized data
        let id = serialized.id;
        // Look up the asset type by name in the manager
        let asset_type = mgr.asset_type_by_name(&serialized.asset_type)?;
        // Upgrade the weak reference to ensure the asset type is still valid
        let asset_type_ptr = asset_type.upgrade()?;
        // Get the serialized data bytes
        let data_bytes = serialized.data;
        // Use the asset type's loader to deserialize the data
        let data = asset_type_ptr.loader().asset_from_bytes(&data_bytes).ok()?;
        Some(Self {
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
