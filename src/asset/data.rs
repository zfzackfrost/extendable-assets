use downcast_rs::{DowncastSync, impl_downcast};

/// Trait for asset data that can be stored in an asset.
///
/// This trait extends `DowncastSync` to allow for safe downcasting to concrete types.
/// Types must explicitly implement this trait to be used as asset data.
pub trait AssetData: DowncastSync {}
impl_downcast!(sync AssetData);
