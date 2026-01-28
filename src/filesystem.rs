use std::path::{Path, PathBuf};

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

/// A filesystem implementation that reads from the native OS filesystem.
///
/// This implementation provides access to files on the local filesystem,
/// with all asset paths resolved relative to a configured root directory.
pub struct NativeFilesystem {
    /// Root directory where all asset paths are resolved relative to
    root_dir: PathBuf,
}
impl NativeFilesystem {
    /// Creates a new native filesystem with the specified root directory.
    ///
    /// # Arguments
    ///
    /// * `root_dir` - The root directory for resolving asset paths
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            root_dir: PathBuf::from(root_dir.as_ref()),
        }
    }
}
impl Filesystem for NativeFilesystem {
    fn read_bytes(&self, asset_path: &str) -> Result<Vec<u8>, FilesystemError> {
        // Resolve the asset path relative to our root directory
        let path = self.root_dir.join(asset_path);

        // Check if the file exists before attempting to read
        if !path.is_file() {
            return Err(FilesystemError::NotFound(asset_path.to_string()));
        }

        // Read the entire file into memory
        let bytes = std::fs::read(path).map_err(FilesystemError::from)?;
        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    /// Tests that the native filesystem can successfully read a test file.
    #[test]
    fn native_fs_read() {
        // Create a filesystem instance rooted at the current directory
        let fs: Arc<dyn Filesystem> =
            Arc::new(NativeFilesystem::new(std::env::current_dir().unwrap()));

        // Read a test file and verify its contents
        let greeting = fs.read_bytes("test_data/hello.txt").unwrap();
        assert_eq!(greeting, b"Hello world\n");
    }
}
