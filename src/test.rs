use crate::*;

use std::sync::Arc;

#[derive(Debug, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
struct TestAssetData {
    value_a: u32,
    value_b: f32,
    value_c: (u8, u8, u8),
}
impl AssetData for TestAssetData {}

struct TestAssetType;
impl AssetType for TestAssetType {
    fn name(&self) -> &str {
        "TestAsset"
    }
    fn loader(&self) -> Box<dyn AssetLoader> {
        struct Loader;
        impl AssetLoader for Loader {
            fn asset_from_bytes(&self, bytes: &[u8]) -> Result<Box<dyn AssetData>, AssetLoadError> {
                let data: TestAssetData = rmp_serde::from_slice(bytes)
                    .map_err(|e| AssetLoadError::Deserialization(anyhow::Error::new(e)))?;
                Ok(Box::new(data))
            }
        }
        Box::new(Loader)
    }

    fn saver(&self) -> Box<dyn AssetSaver> {
        struct Saver;
        impl AssetSaver for Saver {
            fn asset_to_bytes(&self, asset: &dyn AssetData) -> Result<Vec<u8>, AssetSaveError> {
                let data = asset
                    .downcast_ref::<TestAssetData>()
                    .ok_or(AssetSaveError::UnsupportedType)?;
                rmp_serde::to_vec(&data)
                    .map_err(|e| AssetSaveError::Serialization(anyhow::Error::new(e)))
            }
        }
        Box::new(Saver)
    }
}

fn setup(asset_id_out: Option<&mut AssetId>, asset_type: bool) -> AssetManager {
    let mgr = AssetManager::new(Arc::new(NativeFilesystem::new("./")));
    if asset_type {
        mgr.register_asset_type(Arc::new(TestAssetType));
    }
    if let Some(asset_id_out) = asset_id_out {
        *asset_id_out = mgr.register_asset(Asset::new(
            mgr.asset_type_by_name("TestAsset")
                .expect("Asset type not found"),
            Box::new(TestAssetData {
                value_a: 42,
                value_b: std::f32::consts::PI,
                value_c: (1, 2, 3),
            }),
        ));
    }
    mgr
}

#[test]
fn register_get_asset_type() {
    let mgr = setup(None, true);
    let asset_type = mgr.asset_type_by_name("TestAsset");
    assert!(asset_type.is_some());
    let asset_type = asset_type.unwrap();
    let name = asset_type.name();
    assert_eq!(name, "TestAsset");
}
#[test]
fn register_get_asset() {
    let mut asset_id: AssetId = 0;
    let mgr = setup(Some(&mut asset_id), true);
    let asset = mgr.asset_by_id(asset_id);
    assert!(asset.is_some());
    let asset = asset.unwrap();
    assert_eq!(asset.id(), asset_id);
}
