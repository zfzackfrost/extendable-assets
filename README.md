# extendable-assets

[![License](https://img.shields.io/github/license/zfzackfrost/extendable-assets?style=flat-square)][LICENSE]
[![Crates.io](https://img.shields.io/crates/v/extendable-assets?style=flat-square)][CratesIO]
[![docs.rs](https://img.shields.io/docsrs/extendable-assets?style=flat-square)][DocsRS]

Asset framework for graphics and games.

## Usage

Here's a basic example of using extendable-assets to manage texture assets:

```rust
use std::sync::Arc;
use extendable_assets::*;

// Define your asset data type
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub pixels: Vec<u8>,
}

// Implement the AssetData trait
impl AssetData for TextureData {}

// Create an asset type that defines how to load/save your assets
pub struct TextureAssetType;

impl AssetType for TextureAssetType {
    fn name(&self) -> &str {
        "Texture"
    }
    
    fn loader(&self) -> Box<dyn AssetDataLoader> {
        struct TextureLoader;
        impl AssetDataLoader for TextureLoader {
            fn asset_from_bytes(
                &self,
                bytes: &[u8],
                _context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Box<dyn AssetData>, AssetLoadError> {
                let data: TextureData = serde_json::from_slice(bytes)
                    .map_err(|e| AssetLoadError::Deserialization(anyhow::Error::new(e)))?;
                Ok(Box::new(data))
            }
        }
        Box::new(TextureLoader)
    }
    
    fn saver(&self) -> Box<dyn AssetDataSaver> {
        struct TextureSaver;
        impl AssetDataSaver for TextureSaver {
            fn asset_to_bytes(
                &self,
                asset: &dyn AssetData,
                _context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Vec<u8>, AssetSaveError> {
                let data = asset
                    .downcast_ref::<TextureData>()
                    .ok_or(AssetSaveError::UnsupportedType)?;
                serde_json::to_vec(&data)
                    .map_err(|e| AssetSaveError::Serialization(anyhow::Error::new(e)))
            }
        }
        Box::new(TextureSaver)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create asset manager with native filesystem
    let mut manager = AssetManager::new(Arc::new(NativeFilesystem::new("assets")));
    
    // Set JSON serialization backend
    manager.set_serialization_backend(Box::new(JsonAssetSerializationBackend));
    
    // Register your asset type
    manager.register_asset_type(Arc::new(TextureAssetType));
    
    // Create and register an asset manually
    let texture_data = TextureData {
        width: 256,
        height: 256,
        format: "RGBA8".to_string(),
        pixels: vec![255; 256 * 256 * 4],
    };
    
    let asset_type = manager.asset_type_by_name("Texture").unwrap();
    let asset = Asset::new(asset_type, Box::new(texture_data));
    let asset_id = manager.register_asset("my_texture", asset);
    
    // Load an asset from file
    let loaded_id = manager.fs_read_and_register_asset("textures/grass.json").await?;
    let loaded_asset = manager.asset_by_id(loaded_id).unwrap();
    
    // Access the asset data
    if let Some(texture) = loaded_asset.data().downcast_ref::<TextureData>() {
        println!("Loaded texture: {}x{} ({})", texture.width, texture.height, texture.format);
    }
    
    Ok(())
}
```

### Key Features

- **Type-safe asset system**: Define custom asset types with compile-time safety
- **Multiple filesystem backends**: Native filesystem, embedded assets, or custom implementations  
- **Flexible serialization**: JSON, MessagePack, or custom serialization backends
- **Async I/O**: Non-blocking asset loading and saving operations
- **Deterministic IDs**: Hash-based asset IDs for consistent references
- **Extensible**: Easy to add new asset types and storage backends

## AI Disclosure

Generative AI has been used to assist in writing documentation and commit messages. The code
in this project has been written by a human.

[CratesIO]: https://crates.io/crates/extendable-assets
[DocsRS]: https://docs.rs/extendable-assets/latest/extendable_assets/
[LICENSE]: https://github.com/zfzackfrost/extendable-assets/blob/main/LICENSE
