mod common;
use common::*;

use rand::Rng;

use std::path::Path;
use std::sync::Arc;

use extendable_assets::*;

fn init_mgr() -> AssetManager {
    let tests_dir = Path::new(&env!("CARGO_MANIFEST_DIR")).join("tests");
    AssetManager::new(Arc::new(NativeFilesystem::new(tests_dir)))
}
fn register_types(mgr: &AssetManager) {
    mgr.register_asset_type(Arc::new(TestAssetType));
}
fn register_assets(mgr: &AssetManager) -> AssetId {
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

#[test]
fn register_get_asset_type() {
    let mgr = init_mgr();
    register_types(&mgr);

    let asset_type = mgr.asset_type_by_name("TestAsset").unwrap();
    let asset_type = asset_type.upgrade().unwrap();
    let name = asset_type.name();
    assert_eq!(name, "TestAsset");
}
#[test]
fn register_get_asset() {
    let mgr = init_mgr();
    register_types(&mgr);
    let asset_id = register_assets(&mgr);

    let asset = mgr.asset_by_id(asset_id).unwrap();
    assert_eq!(asset.id(), asset_id);
}
