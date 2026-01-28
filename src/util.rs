use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

/// A specialized hasher optimized for u64 keys.
///
/// This hasher is designed specifically for hashing u64 values efficiently
/// by using a simple multiplication with a large prime number.
#[derive(Clone, Default)]
pub(crate) struct U64Hasher {
    /// The accumulated hash value
    value: u64,
}
impl Hasher for U64Hasher {
    /// Returns the final hash value.
    fn finish(&self) -> u64 {
        self.value
    }

    /// Generic byte writing is not implemented for this specialized hasher.
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }

    /// Optimized hashing for u64 values using prime multiplication.
    fn write_u64(&mut self, i: u64) {
        // Large prime number close to 2^63 for good distribution
        const PRIME: u64 = (1 << 63) - 25;
        self.value = i.wrapping_mul(PRIME);
    }
}
/// Hash builder for creating U64Hasher instances.
pub(crate) type U64HahserBuilder = BuildHasherDefault<U64Hasher>;

/// A HashMap optimized for u64 keys using the specialized U64Hasher.
///
/// This provides better performance for asset ID lookups compared to the default hasher.
pub(crate) type U64HashMap<V> = HashMap<u64, V, U64HahserBuilder>;

#[cfg(test)]
mod test {
    use super::*;

    /// Test that the U64HashMap correctly stores and retrieves values.
    #[test]
    fn hash_map_u64() {
        use rand::prelude::*;
        use rand::rng;

        // Generate a large number of random u64 keys for testing
        let mut nums = vec![0u64; 1 << 16];
        rng().fill(&mut nums[..]);

        // Create a hash map with our optimized hasher
        let mut hash_map =
            U64HashMap::<u8>::with_capacity_and_hasher(1 << 16, U64HahserBuilder::new());

        for n in nums {
            // Insert a random value for each key
            let value = rng().random();
            hash_map.insert(n, value);

            // Verify that the value can be retrieved correctly
            assert!(matches!(hash_map.get(&n), Some(v) if *v == value));
        }
    }
}
