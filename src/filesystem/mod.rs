mod fallback;
pub use fallback::*;

#[cfg(feature = "fs-native")]
mod native;
#[cfg(feature = "fs-native")]
pub use native::*;

use async_trait::async_trait;

/// Errors that can occur during filesystem operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FilesystemError {
    /// Standard I/O error occurred.
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    /// Writing operations are not supported by this filesystem implementation.
    #[error("Writing is unsupported on this filesystem")]
    WriteUnsupported,
    /// The requested asset file was not found.
    #[error("Asset not found: {0}")]
    NotFound(String),
    /// Any other filesystem-related error.
    #[error(transparent)]
    Other(anyhow::Error),
}

/// Abstraction for reading files from different storage backends.
///
/// This trait allows the asset system to work with different filesystem
/// implementations, such as native filesystem, network storage, or embedded assets.
/// All filesystem operations are async to support non-blocking I/O.
#[async_trait]
pub trait Filesystem: Send + Sync {
    /// Asynchronously reads the contents of an asset file as raw bytes.
    ///
    /// This method performs non-blocking I/O to read the entire file into memory.
    /// For large files, consider implementing streaming or chunked reading.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path to the asset file
    ///
    /// # Returns
    ///
    /// A future that resolves to the file contents as bytes, or an error if the file could not be read.
    async fn read_bytes(&self, asset_path: &str) -> Result<Vec<u8>, FilesystemError>;

    /// Asynchronously writes raw bytes to an asset file.
    ///
    /// This method provides a default implementation that returns `WriteUnsupported`.
    /// Filesystem implementations that support writing should override this method.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path where the asset file should be written
    /// * `data` - The raw bytes to write to the file
    ///
    /// # Returns
    ///
    /// A future that resolves to success or an error if the write operation fails.
    /// The default implementation always returns `WriteUnsupported`.
    #[allow(unused_variables)]
    async fn write_bytes(&self, asset_path: &str, data: &[u8]) -> Result<(), FilesystemError> {
        Err(FilesystemError::WriteUnsupported)
    }
}
