mod common;
use common::*;

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
