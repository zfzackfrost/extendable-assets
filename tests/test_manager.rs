mod common;
use common::*;

use std::sync::Arc;

use extendable_assets::*;

fn setup(asset_id_out: Option<&mut AssetId>, asset_type: bool) -> AssetManager {
    let cwd = std::env::current_dir().unwrap();
    let mgr = AssetManager::new(Arc::new(NativeFilesystem::new(cwd)));
    if asset_type {
        mgr.register_asset_type(Arc::new(TestAssetType));
    }
    if let Some(asset_id_out) = asset_id_out {
        *asset_id_out = mgr.register_asset(
            "test_asset_01",
            Asset::new(
                mgr.asset_type_by_name("TestAsset")
                    .expect("Asset type not found"),
                Box::new(TestAssetData {
                    value_a: 42,
                    value_b: std::f32::consts::PI,
                    value_c: (1, 2, 3),
                }),
            ),
        );
    }
    mgr
}

#[test]
fn register_get_asset_type() {
    let mgr = setup(None, true);
    let asset_type = mgr.asset_type_by_name("TestAsset");
    assert!(asset_type.is_some());
    let asset_type = asset_type.unwrap();
    let asset_type = asset_type.upgrade().unwrap();
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
