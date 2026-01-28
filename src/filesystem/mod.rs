mod native;
pub use native::*;

/// Errors that can occur during filesystem operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FilesystemError {
    /// Standard I/O error occurred.
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
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
pub trait Filesystem: Send + Sync {
    /// Reads the contents of an asset file as raw bytes.
    ///
    /// # Arguments
    ///
    /// * `asset_path` - The path to the asset file
    ///
    /// # Returns
    ///
    /// The file contents as bytes, or an error if the file could not be read.
    fn read_bytes(&self, asset_path: &str) -> Result<Vec<u8>, FilesystemError>;
}
