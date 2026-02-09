use extendable_assets::*;

use std::sync::Arc;

#[allow(unused)]
pub struct TestContext {
    pub value: String,
}
impl AssetManagerContext for TestContext {}

#[derive(Clone, Debug, PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(unused)]
pub struct TestContextAssetData {
    pub value: u32,
    #[serde(skip)]
    pub ctx: Option<String>,
}
impl AssetData for TestContextAssetData {}

#[allow(unused)]
pub struct TestContextAssetType;
impl AssetType for TestContextAssetType {
    fn name(&self) -> &str {
        "TestContextAsset"
    }
    fn loader(&self) -> Box<dyn AssetDataLoader> {
        struct Loader;
        impl AssetDataLoader for Loader {
            fn asset_from_bytes(
                &self,
                bytes: &[u8],
                context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Box<dyn AssetData>, AssetLoadError> {
                let mut data: TestContextAssetData = serde_json::from_slice(bytes)
                    .map_err(|e| AssetLoadError::Deserialization(anyhow::Error::new(e)))?;

                let context = context.unwrap();
                let context = context
                    .downcast_arc::<TestContext>()
                    .map_err(|_| AssetLoadError::Other(anyhow::anyhow!("Invalid downcast")))?;
                data.ctx = Some(context.value.clone());
                Ok(Box::new(data))
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
                context: Option<Arc<dyn AssetManagerContext>>,
            ) -> Result<Vec<u8>, AssetSaveError> {
                let context = context.unwrap();
                context
                    .downcast_arc::<TestContext>()
                    .map_err(|_| AssetSaveError::Other(anyhow::anyhow!("Invalid downcast")))?;

                let data = asset
                    .downcast_ref::<TestContextAssetData>()
                    .ok_or(AssetSaveError::UnsupportedType)?;
                serde_json::to_vec(&data)
                    .map_err(|e| AssetSaveError::Serialization(anyhow::Error::new(e)))
            }
        }
        Box::new(Saver)
    }
}
