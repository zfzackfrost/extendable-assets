use rust_embed::EmbeddedFile;

use crate::filesystem::{Filesystem, FilesystemError};

use async_trait::async_trait;

/// Trait for providing access to embedded files.
///
/// This trait exists because `rust-embed` types are not dyn-compatible,
/// so we need this abstraction layer to allow dynamic dispatch over
/// different embedded file collections.
pub trait EmbedFilesystemProvider: Send + Sync {
    /// Retrieves an embedded file by its path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the embedded file
    ///
    /// # Returns
    ///
    /// Returns the embedded file if found, or `None` if the path doesn't exist.
    fn get(&self, path: &str) -> Option<EmbeddedFile>;
}
/// A filesystem implementation that reads from embedded files.
///
/// This filesystem provides read-only access to files that have been embedded
/// into the binary at compile time using `rust-embed`.
pub struct EmbedFilesystem {
    /// The provider that handles access to the embedded files
    provider: Box<dyn EmbedFilesystemProvider>,
    root_dir: String,
}
impl EmbedFilesystem {
    /// Creates a new embedded filesystem with the given provider.
    ///
    /// # Arguments
    ///
    /// * `provider` - The embedded file provider to use for file access
    ///
    /// # Returns
    ///
    /// A new `EmbedFilesystem` instance.
    #[inline]
    pub fn new(provider: Box<dyn EmbedFilesystemProvider>) -> Self {
        Self {
            provider,
            root_dir: String::new(),
        }
    }
    pub fn with_root_dir(mut self, root_dir: &str) -> Self {
        self.root_dir = if root_dir.is_empty() {
            String::new()
        } else {
            root_dir.trim_end_matches(['/', '\\']).to_string() + "/"
        };
        self
    }
}
#[async_trait]
impl Filesystem for EmbedFilesystem {
    /// Reads the contents of an embedded file as bytes.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path to the asset file to read
    ///
    /// # Returns
    ///
    /// The file contents as a `Vec<u8>` on success, or a `FilesystemError`
    /// if the file is not found.
    ///
    /// # Errors
    ///
    /// Returns `FilesystemError::NotFound` if the requested file path
    /// does not exist in the embedded files.
    async fn read_bytes(&self, asset_path: &str) -> Result<Vec<u8>, FilesystemError> {
        let prefixed_path = self.root_dir.clone() + asset_path;
        // Look up the embedded file using the provider
        let embedded = self
            .provider
            .get(&prefixed_path)
            .ok_or_else(|| FilesystemError::NotFound(prefixed_path))?;

        // Convert the embedded file data to owned bytes
        Ok(embedded.data.into_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    /// Test implementation of EmbedFilesystemProvider
    #[derive(rust_embed::Embed)]
    #[folder = "$CARGO_MANIFEST_DIR/tests"]
    struct TestEmbedFsProvider;
    impl EmbedFilesystemProvider for TestEmbedFsProvider {
        /// Retrieves an embedded test file by its path.
        fn get(&self, path: &str) -> Option<EmbeddedFile> {
            // Delegate to the rust-embed generated static method
            TestEmbedFsProvider::get(path)
        }
    }

    /// Test that the embedded filesystem can successfully read file contents.
    ///
    /// This test verifies that:
    /// 1. The EmbedFilesystem can be constructed with a test provider
    /// 2. The filesystem can locate and read an embedded test file
    /// 3. The file contents are returned correctly as bytes
    /// 4. The async interface works properly with pollster for blocking execution
    ///
    /// The test uses a known test file `test_data_0/hello.txt` that should
    /// contain the text "Hello world\n" to verify the read operation.
    #[test]
    fn read_bytes() {
        // Create an embedded filesystem using our test provider
        let fs: Arc<dyn Filesystem> = Arc::new(EmbedFilesystem::new(Box::new(TestEmbedFsProvider)));

        // Read the test file contents and verify they match expected value
        let greeting = pollster::block_on(fs.read_bytes("test_data_0/hello.txt")).unwrap();
        assert_eq!(greeting, b"Hello world\n");
    }

    #[test]
    fn read_bytes_with_root() {
        // Create an embedded filesystem using our test provider
        // Add a root directory, with a bunch of slashes at the end (they should be removed
        // automatically)
        let fs: Arc<dyn Filesystem> = Arc::new(
            EmbedFilesystem::new(Box::new(TestEmbedFsProvider)).with_root_dir("test_data_1///"),
        );

        // Read the test file contents and verify they match expected value
        let greeting = pollster::block_on(fs.read_bytes("hello.txt")).unwrap();
        assert_eq!(greeting, b"Hello earth\n");
    }
}
