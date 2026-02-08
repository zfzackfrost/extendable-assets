mod context;
#[allow(unused_imports)]
pub use context::*;

use extendable_assets::*;
use rand::Rng;
use rand::RngExt;
use rand::distr::{Distribution, StandardUniform};

use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TestAssetData {
    pub value_a: u32,
    pub value_b: f32,
    pub value_c: (u8, u8, u8),
}
impl AssetData for TestAssetData {}

impl Distribution<TestAssetData> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TestAssetData {
        TestAssetData {
            value_a: rng.random(),
            value_b: rng.random(),
            value_c: (rng.random(), rng.random(), rng.random()),
        }
    }
}

#[allow(dead_code)]
pub struct TestAssetType;
impl AssetType for TestAssetType {
    fn name(&self) -> &str {
        "TestAsset"
    }
    fn loader(&self) -> Box<dyn AssetLoader> {
        struct Loader;
        impl AssetLoader for Loader {
            fn asset_from_bytes(
                &self,
                bytes: &[u8],
                _context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Box<dyn AssetData>, AssetLoadError> {
                let data: TestAssetData = serde_json::from_slice(bytes)
                    .map_err(|e| AssetLoadError::Deserialization(anyhow::Error::new(e)))?;
                Ok(Box::new(data))
            }
        }
        Box::new(Loader)
    }

    fn saver(&self) -> Box<dyn AssetSaver> {
        struct Saver;
        impl AssetSaver for Saver {
            fn asset_to_bytes(
                &self,
                asset: &dyn AssetData,
                _context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Vec<u8>, AssetSaveError> {
                let data = asset
                    .downcast_ref::<TestAssetData>()
                    .ok_or(AssetSaveError::UnsupportedType)?;
                serde_json::to_vec(&data)
                    .map_err(|e| AssetSaveError::Serialization(anyhow::Error::new(e)))
            }
        }
        Box::new(Saver)
    }
}

#[allow(unused)]
pub fn init_mgr() -> AssetManager {
    let tests_dir = Path::new(&env!("CARGO_MANIFEST_DIR")).join("tests");
    AssetManager::new(Arc::new(NativeFilesystem::new(tests_dir)))
}
#[allow(unused)]
pub fn register_types(mgr: &AssetManager) {
    mgr.register_asset_type(Arc::new(TestAssetType));
    mgr.register_asset_type(Arc::new(TestContextAssetType));
}
#[allow(unused)]
pub fn register_assets(mgr: &AssetManager) -> AssetId {
    let data: TestAssetData = rand::rng().random();
    mgr.register_asset(
        "test_asset_01",
        Asset::new(
            mgr.asset_type_by_name("TestAsset")
                .expect("Asset type not found"),
            Box::new(data),
        ),
    )
}
