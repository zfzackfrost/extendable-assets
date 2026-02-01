use std::path::{Path, PathBuf};

use crate::filesystem::{Filesystem, FilesystemError};

use async_trait::async_trait;

/// A filesystem implementation that reads from the native OS filesystem.
///
/// This implementation provides async access to files on the local filesystem,
/// with all asset paths resolved relative to a configured root directory.
/// Currently uses blocking I/O operations wrapped in async functions.
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

    /// Returns a reference to the root directory path.
    ///
    /// This is the directory that all asset paths are resolved relative to.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}
#[async_trait]
impl Filesystem for NativeFilesystem {
    async fn read_bytes(&self, asset_path: &str) -> Result<Vec<u8>, FilesystemError> {
        // Resolve the asset path relative to our root directory
        let path = self.root_dir.join(asset_path);

        // Check if the file exists before attempting to read
        if !path.is_file() {
            return Err(FilesystemError::NotFound(asset_path.to_string()));
        }

        // Read the entire file into memory asynchronously
        // Note: Currently using blocking I/O - could be improved with tokio::fs for true async I/O
        let bytes = std::fs::read(path).map_err(FilesystemError::from)?;
        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    /// Tests that the native filesystem can successfully read a test file asynchronously.
    ///
    /// Uses pollster to block on the async operation for testing purposes.
    #[test]
    fn read_bytes() {
        let tests_dir = Path::new(&env!("CARGO_MANIFEST_DIR")).join("tests");

        // Create a filesystem instance rooted at the "tests" directory
        let fs: Arc<dyn Filesystem> = Arc::new(NativeFilesystem::new(tests_dir));

        // Read a test file asynchronously and verify its contents
        // Using pollster::block_on to wait for the async operation in a sync test
        let greeting = pollster::block_on(fs.read_bytes("test_data_0/hello.txt")).unwrap();
        assert_eq!(greeting, b"Hello world\n");
    }
}
