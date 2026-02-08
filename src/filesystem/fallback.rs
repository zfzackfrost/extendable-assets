use std::sync::Arc;

use async_trait::async_trait;

use crate::filesystem::{Filesystem, FilesystemError};

/// A filesystem implementation that tries multiple fallback filesystems in order.
///
/// `FallbackFilesystem` allows chaining multiple filesystem implementations together,
/// attempting operations on each one in sequence until one succeeds. This is useful
/// for implementing layered asset loading where assets might be located in different
/// locations (e.g., first check a mod directory, then fall back to base game assets).
pub struct FallbackFilesystem {
    /// Ordered list of filesystem implementations to try.
    ///
    /// When any filesystem operation is requested, each filesystem in this list
    /// is tried in order until one succeeds or all have failed.
    fallbacks: Vec<Arc<dyn Filesystem>>,
}

impl FallbackFilesystem {
    /// Creates a new `FallbackFilesystem` with the given list of fallback filesystems.
    ///
    /// The filesystems will be tried in the order they appear in the vector.
    /// Earlier filesystems have priority over later ones.
    ///
    /// # Arguments
    ///
    /// * `fallbacks` - A vector of filesystem implementations to use as fallbacks
    ///
    /// # Example
    ///
    /// ```
    /// # use extendable_assets::{ FallbackFilesystem, NativeFilesystem };
    /// # use std::sync::Arc;
    /// let mod_fs = Arc::new(NativeFilesystem::new("mods/"));
    /// let base_fs = Arc::new(NativeFilesystem::new("assets/"));
    /// let fallback = FallbackFilesystem::new(vec![mod_fs, base_fs]);
    /// ```
    #[inline]
    pub fn new(fallbacks: Vec<Arc<dyn Filesystem>>) -> Self {
        // Create a new instance with the provided fallback filesystems
        Self { fallbacks }
    }
}

#[async_trait]
impl Filesystem for FallbackFilesystem {
    /// Attempts to read bytes from the asset path using the fallback filesystems.
    ///
    /// Tries each filesystem in order until one successfully reads the asset.
    /// Returns the bytes from the first successful read operation.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path to the asset to read
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - The asset bytes from the first successful filesystem
    /// * `Err(FilesystemError::NotFound)` - If all filesystems fail to find the asset
    async fn read_bytes(&self, asset_path: &str) -> Result<Vec<u8>, FilesystemError> {
        // Try each fallback filesystem in order
        for f in &self.fallbacks {
            let r = f.read_bytes(asset_path).await;
            // Return immediately on the first successful read
            if r.is_ok() {
                return r;
            }
            // Continue to next fallback if this one failed
        }
        // All fallbacks failed, asset not found in any filesystem
        Err(FilesystemError::NotFound(asset_path.to_string()))
    }

    /// Attempts to write bytes to the asset path using the fallback filesystems.
    ///
    /// Tries each filesystem in order until one successfully writes the asset.
    /// The write operation stops at the first successful filesystem.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path where the asset should be written
    /// * `data` - The bytes to write
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If any filesystem successfully writes the data
    /// * `Err(FilesystemError::WriteUnsupported)` - If all filesystems fail to write
    async fn write_bytes(&self, asset_path: &str, data: &[u8]) -> Result<(), FilesystemError> {
        // Try each fallback filesystem in order
        for f in &self.fallbacks {
            let r = f.write_bytes(asset_path, data).await;
            // Return immediately on the first successful write
            if r.is_ok() {
                return r;
            }
            // Continue to next fallback if this one failed
        }
        // All fallbacks failed, unable to write to any filesystem
        Err(FilesystemError::WriteUnsupported)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::filesystem::NativeFilesystem;
    use std::path::Path;
    use std::sync::Arc;

    /// Tests the fallback filesystem's ability to read files from multiple sources.
    ///
    /// Verifies that:
    /// - Files are read from the first filesystem that contains them
    /// - The fallback mechanism correctly tries filesystems in order
    /// - Different files can be sourced from different fallback filesystems
    #[test]
    fn read_bytes() {
        // Set up test directories relative to the cargo manifest directory
        let tests_dir = Path::new(&env!("CARGO_MANIFEST_DIR")).join("tests");
        let data_0_dir = tests_dir.join("test_data_0");
        let data_1_dir = tests_dir.join("test_data_1");

        // Create two separate filesystem instances for different test data directories
        let data_0: Arc<dyn Filesystem> = Arc::new(NativeFilesystem::new(data_0_dir));
        let data_1: Arc<dyn Filesystem> = Arc::new(NativeFilesystem::new(data_1_dir));

        // Create fallback filesystem with data_1 as primary and data_0 as fallback
        // This order is intentional to test the priority system
        let fs: Arc<dyn Filesystem> = Arc::new(FallbackFilesystem::new(vec![data_1, data_0]));

        // Test reading files that exist in different fallback locations
        let hello = pollster::block_on(fs.read_bytes("hello.txt")).unwrap();
        let goodbye = pollster::block_on(fs.read_bytes("goodbye.txt")).unwrap();
        let word = pollster::block_on(fs.read_bytes("word.txt")).unwrap();

        // Verify that files are read correctly from their respective sources
        // hello.txt should come from data_1 (first priority)
        assert_eq!(hello, b"Hello earth\n");
        // goodbye.txt and word.txt should come from the first source that contains them
        assert_eq!(goodbye, b"Goodbye world\n");
        assert_eq!(word, b"assets\n");

        // Test that non-existent files return NotFound error after trying all fallbacks
        let missing = pollster::block_on(fs.read_bytes("MISSING"));
        assert!(matches!(missing, Err(FilesystemError::NotFound(_))));
    }
}
