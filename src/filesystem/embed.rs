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
    /// Optional root directory prefix to prepend to all file paths.
    /// When set, all file lookups will be prefixed with this directory path.
    /// Trailing slashes are automatically normalized.
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
    /// Sets the root directory for this filesystem.
    ///
    /// When a root directory is set, all file path lookups will be prefixed
    /// with this directory path. Trailing slashes and backslashes are
    /// automatically normalized to ensure consistent path formatting.
    ///
    /// # Arguments
    ///
    /// * `root_dir` - The root directory path to use as a prefix
    ///
    /// # Returns
    ///
    /// Self with the root directory configured
    ///
    /// # Examples
    ///
    /// ```
    /// # use extendable_assets::EmbedFilesystem;
    /// # use extendable_assets::EmbedFilesystemProvider;
    /// struct MockProvider;
    /// impl EmbedFilesystemProvider for MockProvider {
    ///     fn get(&self, _path: &str) -> Option<rust_embed::EmbeddedFile> { None }
    /// }
    /// let fs = EmbedFilesystem::new(Box::new(MockProvider))
    ///     .with_root_dir("assets/");
    /// ```
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

    /// Test implementation of EmbedFilesystemProvider that embeds test files.
    ///
    /// This struct uses the `rust_embed::Embed` derive macro to embed all files
    /// from the `tests/` directory at compile time, making them available for
    /// testing the embedded filesystem functionality.
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

    /// Test that the embedded filesystem works correctly with a root directory.
    ///
    /// This test verifies that:
    /// 1. The root directory feature works correctly with trailing slashes
    /// 2. Path normalization removes excess trailing slashes
    /// 3. Files can be accessed using relative paths when a root is set
    /// 4. The correct file contents are returned from the prefixed path
    ///
    /// The test uses `test_data_1/hello.txt` as the target file by setting
    /// `test_data_1` as the root directory and accessing `hello.txt` directly.
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
