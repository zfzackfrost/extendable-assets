mod common;
use common::*;

use rand::prelude::*;

#[test]
fn asset_data_eq() {
    let mut rng = rand::rng();
    let a: TestAssetData = rng.random();
    let b = a.clone();
    assert_eq!(a, b);
}
#[test]
fn asset_data_ne() {
    let mut rng = rand::rng();
    let a: TestAssetData = rng.random();
    let b: TestAssetData = rng.random();
    assert_ne!(a, b);
}
