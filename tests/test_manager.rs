mod common;
use common::*;

use std::sync::Weak;

use extendable_assets::*;

fn assert_asset_type_name(weak_asset_type: Weak<dyn AssetType>, expected_name: &str) {
    let asset_type = weak_asset_type.upgrade().unwrap();
    let name = asset_type.name();
    assert_eq!(name, expected_name);
}

#[test]
fn register_get_asset_type() {
    let mgr = init_mgr();
    register_types(&mgr);

    let asset_type_0 = mgr.asset_type_by_name("TestAsset").unwrap();
    let asset_type_1 = mgr.asset_type_by_name("TestContextAsset").unwrap();

    assert_asset_type_name(asset_type_0, "TestAsset");
    assert_asset_type_name(asset_type_1, "TestContextAsset");
}
#[test]
fn register_get_asset() {
    let mgr = init_mgr();
    register_types(&mgr);
    let asset_id = register_assets(&mgr);

    let asset = mgr.asset_by_id(asset_id).unwrap();
    assert_eq!(asset.id(), asset_id);
}
