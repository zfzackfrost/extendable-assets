mod common;
use common::*;

use rand::prelude::*;

#[test]
fn asset_data_eq() {
    let mut rng = rand::rng();
    let a = TestAssetData {
        value_a: rng.random(),
        value_b: rng.random(),
        value_c: (rng.random(), rng.random(), rng.random()),
    };
    let b = a.clone();
    assert_eq!(a, b);
}
#[test]
fn asset_data_ne() {
    let mut rng = rand::rng();
    let a = TestAssetData {
        value_a: rng.random(),
        value_b: rng.random(),
        value_c: (rng.random(), rng.random(), rng.random()),
    };
    let b = TestAssetData {
        value_a: a.value_a.wrapping_add(1),
        value_b: a.value_b + 1.0,
        value_c: (
            a.value_c.0.wrapping_add(1),
            a.value_c.1.wrapping_add(1),
            a.value_c.2.wrapping_add(1),
        ),
    };
    assert_ne!(a, b);
}
