use std::collections::HashMap;
use std::sync::{Arc, Weak};

use downcast_rs::{DowncastSync, impl_downcast};
use parking_lot::Mutex;

use xxhash_rust::const_xxh3::const_custom_default_secret;
use xxhash_rust::xxh3::xxh3_64_with_secret;

use crate::asset::{Asset, AssetId};
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
    assets: Mutex<U64HashMap<Arc<Asset>>>,
    /// Filesystem abstraction for reading and writing asset files
    filesystem: Arc<dyn Filesystem>,
    /// Optional context for providing additional state to the asset manager
    context: Option<Arc<dyn AssetManagerContext>>,
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
    /// Encodes an asset path using percent-encoding for URI safety.
    ///
    /// Applies RFC 3986 percent-encoding to asset paths, preserving forward slashes
    /// and unreserved characters while encoding everything else. This ensures asset
    /// paths are safe for use in URIs and filesystem operations.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The asset path to encode
    ///
    /// # Returns
    ///
    /// A percent-encoded version of the asset path
    #[inline]
    pub fn encode_asset_path(&self, asset_path: &str) -> String {
        asset_path
            .chars()
            .map(|c| {
                match c {
                    // Unreserved characters (RFC 3986)
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                    // Preserve forward slashes
                    '/' => c.to_string(),
                    // Everything else gets percent-encoded
                    _ => {
                        let mut buf = [0; 4];
                        let encoded = c.encode_utf8(&mut buf);
                        let mut strs = encoded.bytes().map(|b| format!("%{:02X}", b));
                        if c.is_ascii() {
                            strs.next_back().unwrap()
                        } else {
                            strs.collect::<String>()
                        }
                    }
                }
            })
            .collect()
    }
    /// Generates a deterministic asset ID from an asset path.
    ///
    /// Uses XXH3 hash with a custom secret to generate consistent IDs
    /// for the same asset path across application restarts.
    #[inline]
    fn gen_asset_id(&self, asset_path: &str) -> AssetId {
        let asset_path: String = self.encode_asset_path(asset_path);

        const SECRET: [u8; 192] = const_custom_default_secret(1111);
        xxh3_64_with_secret(asset_path.as_bytes(), &SECRET)
    }
    /// Registers an asset with the manager and assigns it a deterministic ID.
    ///
    /// The asset ID is generated from the asset path using hash-based generation,
    /// ensuring the same path always produces the same ID.
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
        let id = self.gen_asset_id(asset_path);
        asset.set_id(id);
        self.assets.lock().insert(id, Arc::new(asset));
        id
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
}

/// Trait for providing additional context to the asset manager.
///
/// This trait allows extending the asset manager with custom state
/// while maintaining type safety through downcasting capabilities.
pub trait AssetManagerContext: DowncastSync {}
impl_downcast!(sync AssetManagerContext);
