use std::path::{Path, PathBuf};

use crate::filesystem::{Filesystem, FilesystemError};

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
    fn read_bytes() {
        // Create a filesystem instance rooted at the current directory
        let fs: Arc<dyn Filesystem> =
            Arc::new(NativeFilesystem::new(std::env::current_dir().unwrap()));

        // Read a test file and verify its contents
        let greeting = fs.read_bytes("test_data/hello.txt").unwrap();
        assert_eq!(greeting, b"Hello world\n");
    }
}
