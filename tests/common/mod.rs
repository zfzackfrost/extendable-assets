use extendable_assets::*;

#[derive(Clone, Debug, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TestAssetData {
    pub value_a: u32,
    pub value_b: f32,
    pub value_c: (u8, u8, u8),
}
impl AssetData for TestAssetData {}

#[allow(dead_code)]
pub struct TestAssetType;
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
