use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::asset::{Asset, AssetId};
use crate::asset_type::AssetType;
use crate::filesystem::Filesystem;
use crate::util::U64HashMap;

/// Central manager for assets in the system.
/// 
/// The asset manager is responsible for:
/// - Registering and retrieving asset types
/// - Managing asset lifecycles and unique IDs
/// - Providing access to the filesystem for asset operations
/// - Thread-safe storage of assets and their metadata
pub struct AssetManager {
    /// Registry of asset types by their string names
    asset_types: Mutex<HashMap<String, Arc<dyn AssetType>>>,
    /// Storage for loaded assets indexed by their unique IDs
    assets: Mutex<U64HashMap<Arc<Asset>>>,
    /// Filesystem abstraction for reading and writing asset files
    filesystem: Arc<dyn Filesystem>,
    /// Counter for generating unique asset IDs
    next_id: Mutex<AssetId>,
}
impl AssetManager {
    /// Creates a new asset manager with the provided filesystem.
    /// 
    /// # Arguments
    /// 
    /// * `filesystem` - The filesystem implementation to use for asset I/O
    #[inline]
    pub fn new(filesystem: Arc<dyn Filesystem>) -> Self {
        Self {
            asset_types: Mutex::new(HashMap::default()),
            assets: Mutex::new(HashMap::default()),
            filesystem,
            // Start asset IDs from 1 (0 is reserved for uninitialized assets)
            next_id: Mutex::new(1),
        }
    }

    /// Retrieves an asset type by its name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the asset type to look up
    /// 
    /// # Returns
    /// 
    /// The asset type if found, or `None` if no asset type with that name is registered.
    #[inline]
    pub fn asset_type_by_name(&self, name: &str) -> Option<Arc<dyn AssetType>> {
        let asset_types = self.asset_types.lock();
        asset_types.get(name).cloned()
    }
    /// Registers a new asset type with the manager.
    /// 
    /// This allows the asset manager to handle assets of this type.
    /// If an asset type with the same name already exists, it will be replaced.
    /// 
    /// # Arguments
    /// 
    /// * `asset_type` - The asset type implementation to register
    pub fn register_asset_type(&self, asset_type: Arc<dyn AssetType>) {
        self.asset_types
            .lock()
            .insert(asset_type.name().to_string(), asset_type);
    }
    /// Retrieves an asset by its unique ID.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The unique identifier of the asset
    /// 
    /// # Returns
    /// 
    /// The asset if found, or `None` if no asset with that ID is registered.
    #[inline]
    pub fn asset_by_id(&self, id: AssetId) -> Option<Arc<Asset>> {
        let assets = self.assets.lock();
        assets.get(&id).cloned()
    }
    /// Generates a unique asset ID.
    /// 
    /// This is an internal method that atomically increments the ID counter.
    #[inline]
    fn gen_asset_id(&self) -> AssetId {
        let mut next_id = self.next_id.lock();
        let id = *next_id;
        *next_id += 1;
        id
    }
    /// Registers an asset with the manager and assigns it a unique ID.
    /// 
    /// # Arguments
    /// 
    /// * `asset` - The asset to register
    /// 
    /// # Returns
    /// 
    /// The unique ID assigned to the asset
    pub fn register_asset(&self, mut asset: Asset) -> AssetId {
        let id = self.gen_asset_id();
        asset.set_id(id);
        self.assets.lock().insert(id, Arc::new(asset));
        id
    }
}
