use rand::Rng;
use rand::RngExt;
use rand::distr::{Distribution, StandardUniform};

use xxhash_rust::const_xxh3::const_custom_default_secret;
use xxhash_rust::xxh3::xxh3_64_with_secret;

use std::hash::Hash;

#[derive(Default, Clone, Copy)]
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

/// Generates a deterministic asset ID from an asset path.
///
/// Encodes the asset path using percent-encoding for URI safety.
/// Applies RFC 3986 percent-encoding to asset paths, preserving forward slashes
/// and unreserved characters while encoding everything else. This ensures asset
/// paths are safe for use in URIs and filesystem operations.
///
/// Uses XXH3 hash with a custom secret to generate consistent IDs
/// for the same asset path across application restarts.
impl From<&str> for AssetId {
    fn from(value: &str) -> Self {
        let value: String = value
            .chars()
            .map(|c| {
                match c {
                    // Unreserved characters (RFC 3986)
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                    // Preserve forward slashes
                    '/' => c.to_string(),
                    // Everything else gets percent-encoded
                    _ => {
                        let mut buf = [0; 4];
                        let encoded = c.encode_utf8(&mut buf);
                        let mut strs = encoded.bytes().map(|b| format!("%{:02X}", b));
                        if c.is_ascii() {
                            strs.next_back().unwrap()
                        } else {
                            strs.collect::<String>()
                        }
                    }
                }
            })
            .collect();
        const SECRET: [u8; 192] = const_custom_default_secret(1111);
        xxh3_64_with_secret(value.as_bytes(), &SECRET).into()
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
