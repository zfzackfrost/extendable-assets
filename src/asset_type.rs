use crate::loader::AssetLoader;
use crate::saver::AssetSaver;

/// Defines how a specific type of asset should be loaded and saved.
///
/// Asset types provide the metadata and behavior for handling different kinds of assets.
/// Each asset type has a unique name and provides factories for creating loaders and savers.
pub trait AssetType: Send + Sync {
    /// Returns the unique name identifier for this asset type.
    fn name(&self) -> &str;

    /// Creates a new loader instance for this asset type.
    ///
    /// The loader is responsible for converting raw bytes into asset data.
    fn loader(&self) -> Box<dyn AssetLoader>;

    /// Creates a new saver instance for this asset type.
    ///
    /// The saver is responsible for converting asset data back to raw bytes.
    fn saver(&self) -> Box<dyn AssetSaver>;
}
