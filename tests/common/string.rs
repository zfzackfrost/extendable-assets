use extendable_assets::*;

use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(unused)]
pub struct TestStringAssetData {
    pub value: String,
}
impl AssetData for TestStringAssetData {}

#[allow(unused)]
pub struct TestStringAssetType;
impl AssetType for TestStringAssetType {
    fn name(&self) -> &str {
        "TestStringAsset"
    }
    fn loader(&self) -> Box<dyn AssetDataLoader> {
        struct Loader;
        impl AssetDataLoader for Loader {
            fn asset_from_bytes(
                &self,
                bytes: &[u8],
                _context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Box<dyn AssetData>, AssetLoadError> {
                let value = String::from_utf8(bytes.to_vec())
                    .map_err(|e| AssetLoadError::Other(anyhow::Error::new(e)))?;
                Ok(Box::new(TestStringAssetData { value }))
            }
        }
        Box::new(Loader)
    }

    fn saver(&self) -> Box<dyn AssetDataSaver> {
        struct Saver;
        impl AssetDataSaver for Saver {
            fn asset_to_bytes(
                &self,
                asset: &dyn AssetData,
                _context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Vec<u8>, AssetSaveError> {
                let data = asset
                    .downcast_ref::<TestStringAssetData>()
                    .ok_or(AssetSaveError::UnsupportedType)?;
                Ok(data.value.clone().into_bytes())
            }
        }
        Box::new(Saver)
    }
}
