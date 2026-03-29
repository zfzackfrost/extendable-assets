//! # Extendable Assets
//!
//! An asset framework for graphics and games that provides flexible asset management,
//! loading, and saving capabilities.
//!
//! ## Example
//!
//! ```rust
//! use std::sync::Arc;
//! use extendable_assets::*;
//!
//! // Define your asset data type
//! #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
//! pub struct TextureData {
//!     pub width: u32,
//!     pub height: u32,
//!     pub format: String,
//!     pub pixels: Vec<u8>,
//! }
//!
//! // Implement the AssetData trait
//! impl AssetData for TextureData {}
//!
//! // Create an asset type that defines how to load/save your assets
//! pub struct TextureAssetType;
//!
//! impl AssetType for TextureAssetType {
//!     fn name(&self) -> &str {
//!         "Texture"
//!     }
//!     
//!     fn loader(&self) -> Box<dyn AssetDataLoader> {
//!         struct TextureLoader;
//!         impl AssetDataLoader for TextureLoader {
//!             fn asset_from_bytes(
//!                 &self,
//!                 bytes: &[u8],
//!                 _context: Option<Arc<dyn AssetManagerContext>>,
//!             ) -> Result<Box<dyn AssetData>, AssetLoadError> {
//!                 let data: TextureData = serde_json::from_slice(bytes)
//!                     .map_err(|e| AssetLoadError::Deserialization(anyhow::Error::new(e)))?;
//!                 Ok(Box::new(data))
//!             }
//!         }
//!         Box::new(TextureLoader)
//!     }
//!     
//!     fn saver(&self) -> Box<dyn AssetDataSaver> {
//!         struct TextureSaver;
//!         impl AssetDataSaver for TextureSaver {
//!             fn asset_to_bytes(
//!                 &self,
//!                 asset: &dyn AssetData,
//!                 _context: Option<Arc<dyn AssetManagerContext>>,
//!             ) -> Result<Vec<u8>, AssetSaveError> {
//!                 let data = asset
//!                     .downcast_ref::<TextureData>()
//!                     .ok_or(AssetSaveError::UnsupportedType)?;
//!                 serde_json::to_vec(&data)
//!                     .map_err(|e| AssetSaveError::Serialization(anyhow::Error::new(e)))
//!             }
//!         }
//!         Box::new(TextureSaver)
//!     }
//! }
//!
//! # tokio_test::block_on(async {
//! // Create asset manager with native filesystem
//! let mut manager = AssetManager::new(Arc::new(NativeFilesystem::new("assets")));
//! 
//! // Set JSON serialization backend
//! manager.set_serialization_backend(Box::new(JsonAssetSerializationBackend));
//! 
//! // Register your asset type
//! manager.register_asset_type(Arc::new(TextureAssetType));
//! 
//! // Create and register an asset manually
//! let texture_data = TextureData {
//!     width: 256,
//!     height: 256,
//!     format: "RGBA8".to_string(),
//!     pixels: vec![255; 256 * 256 * 4],
//! };
//! 
//! let asset_type = manager.asset_type_by_name("Texture").unwrap();
//! let asset = Asset::new(asset_type, Box::new(texture_data));
//! let asset_id = manager.register_asset("my_texture", asset);
//!
//! // Get the asset back
//! let retrieved_asset = manager.asset_by_id(asset_id).unwrap();
//! if let Some(texture) = retrieved_asset.data().downcast_ref::<TextureData>() {
//!     assert_eq!(texture.width, 256);
//!     assert_eq!(texture.height, 256);
//! }
//! # });
//! ```

mod asset;
mod asset_type;
mod filesystem;
mod loader;
mod manager;
mod saver;
mod util;

/// Third-party re-exports for external crates used by this library.
pub mod third_party;

pub use asset::*;
pub use asset_type::*;
pub use filesystem::*;
pub use loader::*;
pub use manager::*;
pub use saver::*;
