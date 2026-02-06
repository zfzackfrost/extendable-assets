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
        Self { provider }
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
        // Look up the embedded file using the provider
        let embedded = self
            .provider
            .get(asset_path)
            .ok_or_else(|| FilesystemError::NotFound(asset_path.to_string()))?;

        // Convert the embedded file data to owned bytes
        Ok(embedded.data.into_owned())
    }
}
