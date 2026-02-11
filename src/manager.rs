use std::collections::HashMap;
use std::sync::{Arc, Weak};

use downcast_rs::{DowncastSync, impl_downcast};
use parking_lot::Mutex;

use crate::asset::{Asset, AssetId, AssetSerializationBackend, NullSerializationBackend};
use crate::asset_type::AssetType;
use crate::filesystem::{Filesystem, FilesystemError};
use crate::util::U64HashMap;

/// Central manager for assets in the system.
///
/// The asset manager is responsible for:
/// - Registering and retrieving asset types
/// - Managing asset lifecycles and hash-based unique IDs
/// - Providing access to the filesystem for asset operations
/// - Thread-safe storage of assets and their metadata
pub struct AssetManager {
    /// Registry of asset types by their string names
    asset_types: Mutex<HashMap<String, Arc<dyn AssetType>>>,
    /// Storage for loaded assets indexed by their unique IDs
    assets: Mutex<U64HashMap<AssetId, Arc<Asset>>>,
    /// Filesystem abstraction for reading and writing asset files
    filesystem: Arc<dyn Filesystem>,
    /// Optional context for providing additional state to the asset manager
    context: Option<Arc<dyn AssetManagerContext>>,
    /// Backend implementation for serializing and deserializing assets
    serialization: Box<dyn AssetSerializationBackend>,
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
            context: None,
            serialization: Box::new(NullSerializationBackend),
        }
    }

    /// Retrieves the current asset manager context.
    ///
    /// # Returns
    ///
    /// The context if one has been set, otherwise `None`.
    #[inline]
    pub fn context(&self) -> Option<Arc<dyn AssetManagerContext>> {
        self.context.clone()
    }
    /// Sets the asset manager context.
    ///
    /// # Arguments
    ///
    /// * `context` - The context implementation to set
    #[inline]
    pub fn set_context(&mut self, context: Arc<dyn AssetManagerContext>) {
        self.context = Some(context);
    }

    /// Sets the serialization backend for the asset manager.
    ///
    /// This allows changing how assets are serialized and deserialized.
    /// The backend determines the format used for asset persistence.
    ///
    /// # Arguments
    ///
    /// * `serialization` - The serialization backend implementation to use
    pub fn set_serialization_backend(&mut self, serialization: Box<dyn AssetSerializationBackend>) {
        self.serialization = serialization;
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
    pub fn asset_type_by_name(&self, name: &str) -> Option<Weak<dyn AssetType>> {
        let asset_types = self.asset_types.lock();
        asset_types.get(name).map(Arc::downgrade)
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

    /// Registers an asset with the manager and assigns it a deterministic ID.
    ///
    /// If the asset's ID is non-zero, the ID is generated from the asset path using
    /// hash-based generation, ensuring the same path always produces the same ID.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path string used to generate the asset ID
    /// * `asset` - The asset to register
    ///
    /// # Returns
    ///
    /// The deterministic ID assigned to the asset
    pub fn register_asset(&self, asset_path: &str, mut asset: Asset) -> AssetId {
        let id = if asset.id() == AssetId::default() {
            let new_id = AssetId::from(asset_path);
            asset.set_id(new_id);
            new_id
        } else {
            asset.id()
        };
        self.assets.lock().insert(id, Arc::new(asset));
        id
    }

    /// Unregisters an asset from the manager.
    ///
    /// Removes the asset with the given ID from the manager's storage.
    /// The asset will no longer be accessible through the manager after this call.
    /// The asset will be de-allocated when all Arc references to it are dropped.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the asset to remove
    ///
    /// # Returns
    ///
    /// `true` if the asset was found and removed, `false` if no asset with that ID existed.
    pub fn unregister_asset(&self, id: AssetId) -> bool {
        self.assets.lock().remove(&id).is_some()
    }

    /// Asynchronously reads the raw bytes of a file from the filesystem.
    ///
    /// This is a convenience method that delegates to the underlying filesystem
    /// for reading asset files as byte arrays. The operation is non-blocking.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path to the file to read
    ///
    /// # Returns
    ///
    /// A future that resolves to the file contents as bytes, or a `FilesystemError` if reading fails.
    #[inline]
    pub async fn fs_read_bytes(&self, asset_path: &str) -> Result<Vec<u8>, FilesystemError> {
        self.filesystem.read_bytes(asset_path).await
    }

    /// Asynchronously reads an asset file and registers it with the manager.
    ///
    /// This method combines file reading with asset deserialization and registration.
    /// It reads the asset file from the filesystem, deserializes it using the configured
    /// serialization backend, and registers the resulting asset with the manager.
    /// If the asset has no ID (ID is 0), it generates one deterministically from the path.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path to the asset file to read and register
    ///
    /// # Returns
    ///
    /// A future that resolves to the asset ID if successful, or an error if reading,
    /// deserialization, or asset creation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be read from the filesystem
    /// - The file contents cannot be deserialized
    /// - Asset creation from the serialized data fails
    pub async fn fs_read_and_register_asset(&self, asset_path: &str) -> anyhow::Result<AssetId> {
        // Read the raw bytes from the filesystem
        let bytes = self.fs_read_bytes(asset_path).await?;

        // Deserialize the bytes into a SerializedAsset structure
        let serialized = self.serialization.deserialize(&bytes[..])?;

        // Store the ID for return and create the full Asset object
        let asset = Asset::from_serialized(self, serialized)?;

        Ok(self.register_asset(asset_path, asset))
    }
}

/// Trait for providing additional context to the asset manager.
///
/// This trait allows extending the asset manager with custom state
/// while maintaining type safety through downcasting capabilities.
pub trait AssetManagerContext: DowncastSync {}
impl_downcast!(sync AssetManagerContext);
