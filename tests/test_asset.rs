mod common;
use common::*;

use rand::Rng;

use extendable_assets::*;

#[test]
fn from_serialized() {
    let mgr = init_mgr();
    register_types(&mgr);

    let asset_type = mgr
        .asset_type_by_name("TestAsset")
        .expect("Asset type not found");
    let asset_type = asset_type.upgrade().unwrap();
    let asset_data: TestAssetData = rand::rng().random();
    let asset_data_bytes = asset_type.saver().asset_to_bytes(&asset_data).unwrap();

    let serialized = SerializedAsset {
        asset_type: "TestAsset".into(),
        id: rand::rng().random(),
        data: asset_data_bytes,
    };

    let id = serialized.id;
    let loaded_asset = Asset::from_serialized(&mgr, serialized).unwrap();
    let loaded_data = loaded_asset.data().downcast_ref::<TestAssetData>().unwrap();
    assert_eq!(*loaded_data, asset_data);
    assert_eq!(loaded_asset.id(), id);
    assert_eq!(
        loaded_asset.asset_type().upgrade().unwrap().name(),
        "TestAsset"
    );
}
