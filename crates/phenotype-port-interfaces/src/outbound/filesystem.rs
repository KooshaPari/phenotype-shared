//! # Filesystem Ports
//!
//! Filesystem ports define file operations.

use crate::error::Result;
use std::path::Path;

/// File system port for file operations.
#[async_trait::async_trait]
pub trait FileSystem: Send + Sync {
    /// Read file contents.
    async fn read(&self, path: &Path) -> Result<Vec<u8>>;

    /// Write file contents.
    async fn write(&self, path: &Path, contents: &[u8]) -> Result<()>;

    /// Delete a file.
    async fn delete(&self, path: &Path) -> Result<()>;

    /// Check if a file exists.
    async fn exists(&self, path: &Path) -> Result<bool>;

    /// List files in a directory.
    async fn list(&self, path: &Path) -> Result<Vec<std::path::PathBuf>>;

    /// Create a directory.
    async fn create_dir(&self, path: &Path) -> Result<()>;
}

/// Extension trait for filesystem with convenience methods.
pub trait FileSystemExt: FileSystem {
    /// Read file as string.
    async fn read_to_string(&self, path: &Path) -> Result<String> {
        let bytes = self.read(path).await?;
        Ok(String::from_utf8(bytes)?)
    }

    /// Write string to file.
    async fn write_string(&self, path: &Path, contents: &str) -> Result<()> {
        self.write(path, contents.as_bytes()).await
    }

    /// Read JSON file.
    async fn read_json<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Result<T> {
        let contents = self.read_to_string(path).await?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Write JSON file.
    async fn write_json<T: serde::Serialize>(&self, path: &Path, value: &T) -> Result<()> {
        let contents = serde_json::to_string_pretty(value)?;
        self.write_string(path, &contents).await
    }
}

impl<T: FileSystem> FileSystemExt for T {}
