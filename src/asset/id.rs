use rand::Rng;
use rand::RngExt;
use rand::distr::{Distribution, StandardUniform};

use std::hash::Hash;

#[derive(Clone, Copy)]
#[derive(PartialEq, PartialOrd, Ord, Eq)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct AssetId(u64);

impl Hash for AssetId {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0)
    }
}

impl From<AssetId> for u64 {
    #[inline]
    fn from(value: AssetId) -> Self {
        value.0
    }
}
impl From<u64> for AssetId {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct HexWrapper(u64);
        impl std::fmt::Debug for HexWrapper {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("0x{:#x}", self.0))
            }
        }
        f.debug_tuple("AssetId").field(&HexWrapper(self.0)).finish()
    }
}

impl Distribution<AssetId> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AssetId {
        AssetId(rng.random())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_test::{Token, assert_tokens};

    #[test]
    fn asset_id_serde() {
        let id: AssetId = rand::rng().random();
        let id_value: u64 = id.into();
        assert_tokens(&id, &[Token::U64(id_value)]);
    }
}
